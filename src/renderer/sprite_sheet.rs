use std::collections::BTreeMap;
use gfx;
use image::RgbaImage;
use cgmath::Vector2;

use direction::{Direction, CardinalDirections, OrdinalDirections};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::common;
use res::input_sprite::{self, InputSpritePixelCoord};
use content::sprite;

const TILES_PER_WALL: u32 = 256;

// one for the top and one for each possible decoration
const MAX_INSTANCES_PER_WALL: u32 = 5;

#[derive(Clone, Copy, Debug)]
pub enum SpriteResolution {
    Simple(u32),
    Wall(u32),
}

impl Default for SpriteResolution {
    fn default() -> Self {
        SpriteResolution::Simple(0)
    }
}

pub struct SpriteTable(Vec<SpriteResolution>);

impl SpriteTable {
    pub fn get(&self, sprite: sprite::Sprite) -> Option<SpriteResolution> {
        self.0.get(sprite as usize).map(Clone::clone)
    }
}

pub struct SpriteSheet<R: gfx::Resources> {
    pub shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub width: u32,
    pub height: u32,
    pub sprite_table: SpriteTable,
}

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    tex_offset: [f32; 2] = "a_TexOffset",
    index: f32 = "a_Index",
    depth: f32 = "a_Depth",
});

gfx_constant_struct!( Locals {
    in_step: [f32; 2] = "u_InStep",
    out_step: [f32; 2] = "u_OutStep",
    tex_size: [f32; 2] = "u_TexSize",
});

gfx_pipeline!( pipe {
    locals: gfx::ConstantBuffer<Locals> = "Locals",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    depth: gfx::DepthTarget<DepthFormat> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

struct SpriteSheetBuilder<R: gfx::Resources> {
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
    input_sprites: Vec<input_sprite::InputSpritePixelCoord>,
    sprite_table: Vec<SpriteResolution>,
    image: RgbaImage,
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    upload: gfx::handle::Buffer<R, Instance>,
    num_instances: usize,
}

impl<R: gfx::Resources> SpriteSheetBuilder<R> {
    fn new<F>(image: RgbaImage, input_sprites: Vec<InputSpritePixelCoord>, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {

        use self::InputSpritePixelCoord::*;

        let mut num_sprites = 0;
        let mut num_instances = 0;

        for sprite in input_sprites.iter() {
            match sprite {
                &Simple { .. } => {
                    num_sprites += 1;
                    num_instances += 1;
                }
                &Wall { .. } => {
                    num_sprites += TILES_PER_WALL;
                    num_instances += TILES_PER_WALL * MAX_INSTANCES_PER_WALL;
                }
            }
        }

        let width = num_sprites * input_sprite::WIDTH_PX;
        let height = input_sprite::HEIGHT_PX;

        let (_, srv, rtv) = factory.create_render_target(width as u16, height as u16)
            .expect("Failed to create render target for sprite sheet");

        let mut sprite_table = Vec::new();
        for _ in 0..sprite::NUM_SPRITES {
            sprite_table.push(SpriteResolution::default());
        }

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_330_sprite_sheet.vert"),
            include_bytes!("shaders/shdr_330_general.frag"),
            pipe::new()).expect("Failed to create pso");

        let vertex_data: Vec<Vertex> = common::QUAD_VERTICES.iter()
            .map(|v| {
                Vertex { pos: *v }
            }).collect();

        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(
                &vertex_data,
                &common::QUAD_INDICES[..]);

        let (img_width, img_height) = image.dimensions();
        let tex_kind = gfx::texture::Kind::D2(img_width as u16, img_height as u16, gfx::texture::AaMode::Single);
        let (_, texture) = factory.create_texture_immutable_u8::<ColourFormat>(tex_kind, &[&image])
            .expect("Failed to create texture");

        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale,
                                           gfx::texture::WrapMode::Tile));

        let (_, _, depth_rtv) = factory.create_depth_stencil(width as u16, height as u16)
            .expect("Failed to create depth stencil");

        let data = pipe::Data {
            locals: factory.create_constant_buffer(1),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(num_instances as usize, factory)
                .expect("Failed to create instance buffer"),
            tex: (texture, sampler),
            out: rtv,
            depth: depth_rtv,
        };

        let bundle = gfx::pso::bundle::Bundle::new(slice, pso, data);

        let upload = factory.create_upload_buffer(num_instances as usize)
            .expect("Failed to create upload buffer");

        SpriteSheetBuilder {
            shader_resource_view: srv,
            width,
            height,
            input_sprites,
            sprite_table,
            image,
            bundle,
            upload,
            num_instances: 0,
        }
    }

    fn populate<F>(&mut self, factory: &mut F)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {

        use self::InputSpritePixelCoord::*;

        let mut mapping = factory.write_mapping(&self.upload)
            .expect("Failed to map upload buffer");

        let mut instance_index = 0;
        let mut sprite_index = 0;

        for input_sprite in self.input_sprites.iter() {
            match input_sprite {
                &Simple { sprite, coord } => {
                    self.sprite_table[sprite as usize] = SpriteResolution::Simple(sprite_index);
                    mapping[instance_index] = Instance {
                        tex_offset: coord.cast().into(),
                        index: sprite_index as f32,
                        depth: 0.0,
                    };
                    sprite_index += 1;
                    instance_index += 1;
                }
                &Wall { sprite, top, ref decorations } => {
                    self.sprite_table[sprite as usize] = SpriteResolution::Wall(sprite_index);
                    for i in 0..TILES_PER_WALL {
                        let instance_offset = Self::populate_wall(&mut mapping[instance_index..],
                                                                  i as u8, top, decorations,
                                                                  sprite_index);
                        instance_index += instance_offset;
                        sprite_index += 1;
                    }
                }
            }
        }

        self.num_instances = instance_index;
        self.bundle.slice.instances = Some((self.num_instances as u32, 0));
    }

    fn populate_wall(mapping: &mut [Instance], neighbour_bits: u8, top: Vector2<u32>,
                     decorations: &BTreeMap<Direction, Vector2<u32>>, sprite_index: u32) -> usize {

        let mut instance_offset = 0;

        mapping[instance_offset] = Instance {
            tex_offset: top.cast().into(),
            index: sprite_index as f32,
            depth: 0.6,
        };

        instance_offset += 1;

        for card in CardinalDirections {
            let dir_bit = 1 << (card.direction() as usize);
            if neighbour_bits & dir_bit == 0 {
                // neighbour is absent
                let decoration = *decorations.get(&card.direction())
                    .expect(format!("Missing decoration for {:?}", card.direction()).as_ref());
                mapping[instance_offset] = Instance {
                    tex_offset: decoration.cast().into(),
                    index: sprite_index as f32,
                    depth: 0.5,
                };
                instance_offset += 1;
            }
        }

        for ord in OrdinalDirections {
            let ord_bit = 1 << (ord.direction() as usize);
            let (card0, card1) = ord.to_cardinals();
            let card_bits = (1 << (card0.direction() as usize)) | (1 << (card1.direction() as usize));

            if neighbour_bits & card_bits == card_bits && neighbour_bits & ord_bit == 0 {
                // both cardinal neighbours are present but ordinal neighbour is absent
                let decoration = *decorations.get(&ord.direction())
                    .expect(format!("Missing decoration for {:?}", ord.direction()).as_ref());
                mapping[instance_offset] = Instance {
                    tex_offset: decoration.cast().into(),
                    index: sprite_index as f32,
                    depth: 0.5,
                };
                instance_offset += 1;
            }
        }

        assert!(instance_offset <= MAX_INSTANCES_PER_WALL as usize);

        instance_offset
    }

    fn draw<C, D>(&self, encoder: &mut gfx::Encoder<R, C>, device: &mut D)
        where C: gfx::CommandBuffer<R>,
              D: gfx::traits::Device<Resources=R, CommandBuffer=C>,
    {
        encoder.clear(&self.bundle.data.out, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.bundle.data.depth, 1.0);
        encoder.copy_buffer(&self.upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instance buffer");

        let tex_dimensions = self.image.dimensions();
        let tex_width = tex_dimensions.0 as f32;
        let tex_height = tex_dimensions.1 as f32;
        let in_step_x = (input_sprite::WIDTH_PX as f32) / tex_width;
        let in_step_y = (input_sprite::HEIGHT_PX as f32) / tex_height;

        let out_step_x = (input_sprite::WIDTH_PX as f32) / (self.width as f32);
        let out_step_y = (input_sprite::HEIGHT_PX as f32) / (self.height as f32);

        encoder.update_constant_buffer(&self.bundle.data.locals, &Locals {
            in_step: [in_step_x, in_step_y],
            out_step: [out_step_x, out_step_y],
            tex_size: [tex_width, tex_height],
        });

        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        encoder.flush(device);
    }

    fn build(self) -> SpriteSheet<R> {
        let Self { shader_resource_view, width, height, sprite_table, .. } = self;
        SpriteSheet {
            shader_resource_view,
            width,
            height,
            sprite_table: SpriteTable(sprite_table),
        }
    }
}

impl<R: gfx::Resources> SpriteSheet<R> {
    pub fn new<C, F, D>(image: RgbaImage, input_sprites: Vec<InputSpritePixelCoord>,
                        factory: &mut F, encoder: &mut gfx::Encoder<R, C>,
                        device: &mut D) -> Self
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
              D: gfx::traits::Device<Resources=R, CommandBuffer=C>,
    {
        let mut builder = SpriteSheetBuilder::new(image, input_sprites, factory);
        builder.populate(factory);
        builder.draw(encoder, device);
        builder.build()
    }

    pub fn sprite_size(&self) -> (f32, f32) {
        ((input_sprite::WIDTH_PX as f32) / (self.width as f32),
         (input_sprite::HEIGHT_PX as f32) / (self.height as f32))
    }

    pub fn get(&self, sprite: sprite::Sprite) -> Option<SpriteResolution> {
        self.sprite_table.get(sprite)
    }
}
