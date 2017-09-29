use std::cmp;
use std::collections::BTreeMap;

use gfx;
use image::RgbaImage;
use cgmath::Vector2;

use direction::{Direction, OrdinalDirections, DirectionBitmap, CardinalDirection};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::common;
use res::input_sprite::{self, InputSprite, InputSpriteLocation};
use content::sprite::{self, Sprite};
use content::health_overlay::{self, HealthOverlay};

// one for each combination of wall neighbours
const TILES_PER_WALL: u32 = 256;

// one for front and one for top
const INSTANCES_PER_WALL_FIT: u32 = 2;

// one for the top and one for each possible decoration
const MAX_INSTANCES_PER_WALL: u32 = 5;

const SIMPLE_DEPTH: f32 = 0.1;
const WALL_TOP_DEPTH: f32 = 0.9;
const WALL_DECORATION_DEPTH: f32 = 0.1;

#[derive(Clone, Copy, Debug)]
pub struct SpriteLocation {
    pub position: f32,
    pub size: Vector2<f32>,
    pub offset: Vector2<f32>,
}

impl Default for SpriteLocation {
    fn default() -> Self {
        SpriteLocation {
            position: 0.0,
            size: Vector2::new(0.0, 0.0),
            offset: Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WallSpriteLocation(SpriteLocation);

impl WallSpriteLocation {
    pub fn base(&self) -> f32 {
        self.0.position
    }
    pub fn position(&self, bitmap: u8) -> f32 {
        self.0.position + self.0.size.x * bitmap as f32
    }
    pub fn size(&self) -> &Vector2<f32> {
        &self.0.size
    }
    pub fn offset(&self) -> &Vector2<f32> {
        &self.0.offset
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpriteResolution {
    Simple(SpriteLocation),
    Wall(WallSpriteLocation),
    WallFit {
        top: SpriteLocation,
        front: SpriteLocation,
    },
}

impl Default for SpriteResolution {
    fn default() -> Self {
        SpriteResolution::Simple(SpriteLocation::default())
    }
}

#[derive(Debug)]
pub struct SpriteTable {
    sprites: Vec<SpriteResolution>,
    health_overlays: Vec<SpriteResolution>,
}

impl SpriteTable {
    fn new(sprites: Vec<SpriteResolution>, health_overlays: Vec<SpriteResolution>) -> Self {
        SpriteTable {
            sprites,
            health_overlays,
        }
    }
    pub fn get_sprite(&self, sprite: Sprite) -> Option<&SpriteResolution> {
        self.sprites.get(sprite as usize)
    }
    pub fn get_health_overlay(&self, health_overlay: HealthOverlay) -> Option<&SpriteResolution> {
        self.health_overlays.get(health_overlay as usize)
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
    in_pix_pos: [f32; 2] = "a_InPixPos",
    out_pix_pos: [f32; 2] = "a_OutPixPos",
    pix_size: [f32; 2] = "a_PixSize",
    depth: f32 = "a_Depth",
});

gfx_constant_struct!( Locals {
    in_tex_size: [f32; 2] = "u_InTexSize",
    out_tex_size: [f32; 2] = "u_OutTexSize",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    locals: gfx::ConstantBuffer<Locals> = "Locals",
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    depth: gfx::DepthTarget<DepthFormat> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

struct SpriteSheetBuilder<R: gfx::Resources> {
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
    input_sprites: Vec<input_sprite::InputSprite>,
    sprite_table: Vec<SpriteResolution>,
    health_overlay_table: Vec<SpriteResolution>,
    image: RgbaImage,
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    upload: gfx::handle::Buffer<R, Instance>,
    num_instances: usize,
}

impl<R: gfx::Resources> SpriteSheetBuilder<R> {
    fn new<F>(image: RgbaImage, input_sprites: Vec<InputSprite>, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {

        let mut num_instances = 0;
        let mut width = input_sprite::WIDTH_PX; // leave room for blank sprite
        let mut height = 0;

        for sprite in input_sprites.iter() {
            use self::InputSprite::*;
            match sprite {
                &Simple { location, .. } => {
                    num_instances += 1;
                    width += location.size.x;
                    height = cmp::max(height, location.size.y);
                }
                &Wall { top, .. } => {
                    num_instances += TILES_PER_WALL * MAX_INSTANCES_PER_WALL;
                    width += top.size.x * TILES_PER_WALL;
                    height = cmp::max(height, top.size.y);
                }
                &WallFit { top, front, .. } => {
                    num_instances += INSTANCES_PER_WALL_FIT;
                    width += top.size.x + front.size.x;
                    height = cmp::max(cmp::max(top.size.y, front.size.y), height);
                }
                &HealthOverlay { location, .. } => {
                    num_instances += 1;
                    width += location.size.x;
                    height = cmp::max(height, location.size.y);
                }
            }
        }

        let (_, srv, rtv) = factory.create_render_target(width as u16, height as u16)
            .expect("Failed to create render target for sprite sheet");

        let mut sprite_table = Vec::new();
        for _ in 0..sprite::NUM_SPRITES {
            sprite_table.push(SpriteResolution::default());
        }

        let mut health_overlay_table = Vec::new();
        for _ in 0..health_overlay::NUM_HEALTH_OVERLAYS {
            health_overlay_table.push(SpriteResolution::default());
        }

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/sprite_sheet.150.vert"),
            include_bytes!("shaders/general.150.frag"),
            pipe::new()).expect("Failed to create pso");

        let vertex_data: Vec<Vertex> = common::QUAD_VERTICES_REFL.iter()
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
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(num_instances as usize, factory)
                .expect("Failed to create instance buffer"),
            locals: factory.create_constant_buffer(1),
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
            health_overlay_table,
            image,
            bundle,
            upload,
            num_instances: 0,
        }
    }

    fn populate<F>(&mut self, factory: &mut F)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let mut mapping = factory.write_mapping(&self.upload)
            .expect("Failed to map upload buffer");

        // leave a blank sprite at the start
        self.sprite_table[Sprite::Blank as usize] = SpriteResolution::Simple(SpriteLocation {
            position: 0.0,
            size: input_sprite::DIMENSIONS.cast().into(),
            offset: Vector2::new(0.0, 0.0),
        });

        // leave room for blank sprite
        let mut sprite_sheet_x = input_sprite::WIDTH_PX;
        let mut instance_index = 0;

        for input_sprite in self.input_sprites.iter() {
            match input_sprite {
                &InputSprite::Simple { sprite, location } => {
                    self.sprite_table[sprite as usize] = SpriteResolution::Simple(SpriteLocation {
                        position: sprite_sheet_x as f32,
                        size: location.size.cast(),
                        offset: location.offset.cast(),
                    });
                    mapping[instance_index] = Instance {
                        in_pix_pos: location.position.cast().into(),
                        out_pix_pos: [sprite_sheet_x as f32, 0.0],
                        pix_size: location.size.cast().into(),
                        depth: SIMPLE_DEPTH,
                    };
                    sprite_sheet_x += location.size.x;
                    instance_index += 1;
                }
                &InputSprite::Wall { sprite, top, ref decorations } => {
                    self.sprite_table[sprite as usize] = SpriteResolution::Wall(WallSpriteLocation(SpriteLocation {
                        position: sprite_sheet_x as f32,
                        size: top.size.cast(),
                        offset: top.offset.cast(),
                    }));
                    for i in 0..TILES_PER_WALL {
                        let instance_offset = Self::populate_wall(&mut mapping[instance_index..],
                                                                  DirectionBitmap::new(i as u8), top,
                                                                  decorations, sprite_sheet_x);
                        sprite_sheet_x += top.size.x;
                        instance_index += instance_offset;
                    }
                }
                &InputSprite::WallFit { sprite, top, front } => {
                    let front_x = sprite_sheet_x;
                    sprite_sheet_x += front.size.x;
                    let top_x = sprite_sheet_x;
                    sprite_sheet_x += top.size.x;

                    self.sprite_table[sprite as usize] = SpriteResolution::WallFit {
                        front: SpriteLocation {
                            position: front_x as f32,
                            size: front.size.cast(),
                            offset: front.offset.cast(),
                        },
                        top: SpriteLocation {
                            position: top_x as f32,
                            size: top.size.cast(),
                            offset: top.offset.cast(),
                        },
                    };

                    mapping[instance_index] = Instance {
                        in_pix_pos: front.position.cast().into(),
                        out_pix_pos: [front_x as f32, 0.0],
                        pix_size: front.size.cast().into(),
                        depth: SIMPLE_DEPTH,
                    };
                    instance_index += 1;

                    mapping[instance_index] = Instance {
                        in_pix_pos: top.position.cast().into(),
                        out_pix_pos: [top_x as f32, 0.0],
                        pix_size: top.size.cast().into(),
                        depth: SIMPLE_DEPTH,
                    };
                    instance_index += 1;
                }
                &InputSprite::HealthOverlay { health_overlay, location } => {
                    self.health_overlay_table[health_overlay as usize] = SpriteResolution::Simple(SpriteLocation {
                        position: sprite_sheet_x as f32,
                        size: location.size.cast(),
                        offset: location.offset.cast(),
                    });
                    mapping[instance_index] = Instance {
                        in_pix_pos: location.position.cast().into(),
                        out_pix_pos: [sprite_sheet_x as f32, 0.0],
                        pix_size: location.size.cast().into(),
                        depth: SIMPLE_DEPTH,
                    };
                    instance_index += 1;
                    sprite_sheet_x += location.size.x;
                }
            }
        }

        self.num_instances = instance_index;
        self.bundle.slice.instances = Some((self.num_instances as u32, 0));
    }

    fn populate_wall(mapping: &mut [Instance], neighbour_bits: DirectionBitmap, top: InputSpriteLocation,
                     decorations: &BTreeMap<Direction, Vector2<u32>>, sprite_sheet_x: u32) -> usize {
        let mut instance_offset = 0;

        mapping[instance_offset] = Instance {
            in_pix_pos: top.position.cast().into(),
            out_pix_pos: [sprite_sheet_x as f32, 0.0],
            pix_size: top.size.cast().into(),
            depth: WALL_TOP_DEPTH,
        };

        instance_offset += 1;
        use self::CardinalDirection::*;
        for cdir in &[North, East, West, South] {
            let dir = cdir.direction();
            if !neighbour_bits.has(dir) {
                // neighbour is absent
                let decoration = *decorations.get(&dir)
                    .expect(format!("Missing decoration for {:?}", dir).as_ref());
                mapping[instance_offset] = Instance {
                    in_pix_pos: decoration.cast().into(),
                    out_pix_pos: [sprite_sheet_x as f32, 0.0],
                    pix_size: top.size.cast().into(),
                    depth: WALL_DECORATION_DEPTH,
                };
                instance_offset += 1;
            }
        }

        for ord in OrdinalDirections {
            let card_bits = ord.cardinal_bitmap();
            if neighbour_bits & card_bits == card_bits && !neighbour_bits.has(ord.direction()) {
                // both cardinal neighbours are present but ordinal neighbour is absent
                let decoration = *decorations.get(&ord.direction())
                    .expect(format!("Missing decoration for {:?}", ord.direction()).as_ref());
                mapping[instance_offset] = Instance {
                    in_pix_pos: decoration.cast().into(),
                    out_pix_pos: [sprite_sheet_x as f32, 0.0],
                    pix_size: top.size.cast().into(),
                    depth: WALL_DECORATION_DEPTH,
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
        encoder.clear(&self.bundle.data.out, [0.0, 0.0, 0.0, 0.0]);
        encoder.clear_depth(&self.bundle.data.depth, 1.0);
        encoder.copy_buffer(&self.upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instance buffer");

        let in_tex_dimensions = self.image.dimensions();
        encoder.update_constant_buffer(&self.bundle.data.locals, &Locals {
            in_tex_size: [in_tex_dimensions.0 as f32, in_tex_dimensions.1 as f32],
            out_tex_size: [self.width as f32, self.height as f32],
        });

        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        encoder.flush(device);
    }

    fn build(self) -> SpriteSheet<R> {
        let Self { shader_resource_view, width, height, sprite_table, health_overlay_table, .. } = self;
        SpriteSheet {
            shader_resource_view,
            width,
            height,
            sprite_table: SpriteTable {
                sprites: sprite_table,
                health_overlays: health_overlay_table,
            },
        }
    }
}

impl<R: gfx::Resources> SpriteSheet<R> {
    pub fn new<C, F, D>(image: RgbaImage, input_sprites: Vec<InputSprite>,
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
}
