use gfx;

use cgmath::{Vector2, ElementWise};

use renderer::sprite_sheet::{SpriteSheet, SpriteResolution};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::common;

use content::DepthType;
use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;

use res::input_sprite;

const NUM_ROWS: u16 = 15;
const HEIGHT_PX: u16 = NUM_ROWS * input_sprite::HEIGHT_PX as u16;
const MAX_NUM_INSTANCES: usize = 4096;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    sprite_sheet_pix_coord: [f32; 2] = "a_SpriteSheetPixCoord",
    out_pix_coord: [f32; 2] = "a_OutPixCoord",
    pix_size: [f32; 2] = "a_PixSize",
    pix_offset: [f32; 2] = "a_PixOffset",
    depth: f32 = "a_Depth",
});

gfx_constant_struct!( Dimensions {
    sprite_sheet_size: [f32; 2] = "u_SpriteSheetSize",
    output_size: [f32; 2] = "u_OutputSize",
});

gfx_constant_struct!( Offset {
    scroll_offset_pix: [f32; 2] = "u_ScrollOffsetPix",
});

gfx_pipeline!( pipe {
    dimensions: gfx::ConstantBuffer<Dimensions> = "Dimensions",
    offset: gfx::ConstantBuffer<Offset> = "Offset",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    upload: gfx::handle::Buffer<R, Instance>,
    sprite_sheet: SpriteSheet<R>,
    width_px: u16,
    height_px: u16,
    num_instances: usize,
}

impl<R: gfx::Resources> TileRenderer<R> {
    pub fn new<F>(sprite_sheet: SpriteSheet<R>,
                  window_width_px: u16,
                  window_height_px: u16,
                  factory: &mut F) -> (Self, gfx::handle::ShaderResourceView<R, [f32; 4]>)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_150_tile_renderer.vert"),
            include_bytes!("shaders/shdr_150_general.frag"),
            pipe::new()).expect("Failed to create pipeline");

        let vertex_data: Vec<Vertex> = common::QUAD_VERTICES_REFL.iter()
            .map(|v| {
                Vertex {
                    pos: *v,
                }
            }).collect();

        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(
                &vertex_data,
                &common::QUAD_INDICES[..]);

        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale,
                                           gfx::texture::WrapMode::Tile));

        let width_px = ((window_width_px as u32 * HEIGHT_PX as u32) / window_height_px as u32) as u16;
        let (_, srv, colour_rtv) = factory.create_render_target(width_px, HEIGHT_PX)
            .expect("Failed to create render target for sprite sheet");
        let (_, _, depth_rtv) = factory.create_depth_stencil(width_px, HEIGHT_PX)
            .expect("Failed to create depth stencil");

        let data = pipe::Data {
            dimensions: factory.create_constant_buffer(1),
            offset: factory.create_constant_buffer(1),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            out_colour: colour_rtv,
            out_depth: depth_rtv,
            tex: (sprite_sheet.shader_resource_view.clone(), sampler),
        };

        (TileRenderer {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            upload: factory.create_upload_buffer(MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            sprite_sheet,
            width_px,
            height_px: HEIGHT_PX,
            num_instances: 0,
        }, srv)
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width_px, self.height_px)
    }

    pub fn init<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.bundle.data.dimensions, &Dimensions {
            sprite_sheet_size: [self.sprite_sheet.width as f32, self.sprite_sheet.height as f32],
            output_size: [self.width_px as f32, self.height_px as f32],
        });
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.clear(&self.bundle.data.out_colour, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.copy_buffer(&self.upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instances");
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }

    pub fn update_offset<C>(&self, player_position: Vector2<f32>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        let mid = (player_position + Vector2::new(0.5, 0.5))
            .mul_element_wise(Vector2::new(input_sprite::WIDTH_PX, input_sprite::HEIGHT_PX).cast());
        let offset = Vector2::new(mid.x - (self.width_px / 2) as f32, mid.y - (self.height_px / 2) as f32);
        encoder.update_constant_buffer(&self.bundle.data.offset, &Offset {
            scroll_offset_pix: offset.into(),
        });
    }

    pub fn update_entities<F>(&mut self, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                              factory: &mut F)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let mut mapper = factory.write_mapping(&self.upload)
            .expect("Failed to map upload buffer");

        let mut mapper_iter_mut = mapper.iter_mut();

        let mut count = 0;

        for (id, position) in entity_store.position.iter() {
            let depth_type = if let Some(depth_type) = entity_store.depth.get(&id) {
                *depth_type
            } else {
                continue;
            };

            let sprite = if let Some(sprite) = entity_store.sprite.get(&id) {
                *sprite
            } else {
                continue;
            };

            let (sprite_position, sprite_size, sprite_offset) =
                if let Some(sprite_resolution) = self.sprite_sheet.get(sprite) {

                match sprite_resolution {
                    &SpriteResolution::Simple(location) => {
                        (location.position, location.size, location.offset)
                    }
                    &SpriteResolution::Wall(location) => {
                        if let Some(sh_cell) = spatial_hash.get_float(*position) {
                            let bitmap = sh_cell.wall_neighbours.bitmap_raw();
                            (location.position(bitmap), *location.size(), *location.offset())
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                continue;
            };

            let scaled_position = position
                .mul_element_wise(Vector2::new(input_sprite::WIDTH_PX, input_sprite::HEIGHT_PX).cast());

            let depth = match depth_type {
                DepthType::Vertical => 1.0 - position.x / spatial_hash.height() as f32,
                DepthType::Horizontal => 1.0,
            };

            if let Some(instance_slot) = mapper_iter_mut.next() {
                *instance_slot = Instance {
                    sprite_sheet_pix_coord: [sprite_position, 0.0],
                    out_pix_coord: scaled_position.into(),
                    pix_size: sprite_size.into(),
                    pix_offset: sprite_offset.into(),
                    depth: depth,
                };
                count += 1;
            } else {
                panic!("Out of instances!");
            }
        }

        self.bundle.slice.instances = Some((count, 0));
        self.num_instances = count as usize;
    }
}
