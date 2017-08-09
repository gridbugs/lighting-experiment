use gfx;

use cgmath::Vector2;

use renderer::sprite_sheet::{SpriteSheet, SpriteResolution};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::common;

use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;

use res::input_sprite;

const MAX_NUM_INSTANCES: usize = 4096;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    sprite_index: f32 = "a_SpriteIndex",
    size: [f32; 2] = "a_SizePix",
    coord: [f32; 2] = "a_CoordPix",
    depth: f32 = "a_Depth",
});

gfx_constant_struct!( Locals {
    output_size: [f32; 2] = "u_OutputSizePix",
    sprite_size: [f32; 2] = "u_SpriteSize",
});

gfx_pipeline!( pipe {
    locals: gfx::ConstantBuffer<Locals> = "Locals",
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
                  rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                  dsv: gfx::handle::DepthStencilView<R, DepthFormat>,
                  factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_150_tile_renderer.vert"),
            include_bytes!("shaders/shdr_150_general.frag"),
            pipe::new()).expect("Failed to create pipeline");

        let vertex_data: Vec<Vertex> = common::QUAD_VERTICES.iter()
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

        let data = pipe::Data {
            locals: factory.create_constant_buffer(1),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            out_colour: rtv,
            out_depth: dsv,
            tex: (sprite_sheet.shader_resource_view.clone(), sampler),
        };


        let (width_px, height_px, ..) = data.out_colour.get_dimensions();

        TileRenderer {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            upload: factory.create_upload_buffer(MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            sprite_sheet,
            width_px,
            height_px,
            num_instances: 0,
        }
    }

    pub fn init<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        let sprite_size = self.sprite_sheet.sprite_size();
        encoder.update_constant_buffer(&self.bundle.data.locals, &Locals {
            output_size: [self.width_px as f32, self.height_px as f32],
            sprite_size: [sprite_size.0, sprite_size.1],
        });
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.copy_buffer(&self.upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instances");
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }

    pub fn update_entities<F>(&mut self, entity_store: &EntityStore, spatial_hash: &SpatialHashTable,
                              factory: &mut F)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {

        let player_id = entity_store.player.iter().next().expect("Failed to find player");
        let player_position = entity_store.position.get(&player_id).expect("Failed to find player position");
        let mid_coord = player_position + Vector2::new(0.5, 0.5);
        let mid = Vector2::new(mid_coord.x * (input_sprite::WIDTH_PX as f32),
                               mid_coord.y * (input_sprite::HEIGHT_PX as f32));

        let mut mapper = factory.write_mapping(&self.upload)
            .expect("Failed to map upload buffer");

        let mut mapper_iter_mut = mapper.iter_mut();

        let half_width = self.width_px / 2;
        let half_height = self.height_px / 2;

        let half_delta = Vector2::new(half_width as f32, half_height as f32);

        let top_left = mid - half_delta;
        let bottom_right = mid + half_delta;

        let top_left_coord = Vector2::new((top_left.x / input_sprite::WIDTH_PX as f32).floor() as u32,
                                          (top_left.y / input_sprite::HEIGHT_PX as f32).floor() as u32);
        let bottom_right_coord = Vector2::new((bottom_right.x / input_sprite::WIDTH_PX as f32).ceil() as u32,
                                              (bottom_right.y / input_sprite::HEIGHT_PX as f32).ceil() as u32);

        let mut count = 0;

        for (id, coord) in entity_store.coord.iter() {

            if coord.x < top_left_coord.x || coord.y < top_left_coord.y ||
                coord.x >= bottom_right_coord.x || coord.y >= bottom_right_coord.y {
                continue;
            }

            let depth = if let Some(depth) = entity_store.depth.get(id) {
                *depth
            } else {
                continue;
            };

            let sprite = if let Some(sprite) = entity_store.sprite.get(id) {
                *sprite
            } else {
                continue;
            };

            let position = if let Some(position) = entity_store.position.get(id) {
                *position
            } else {
                coord.cast()
            };

            let sprite_index = if let Some(sprite_resolution) = self.sprite_sheet.get(sprite) {
                match sprite_resolution {
                    SpriteResolution::Simple(index) => index,
                    SpriteResolution::Wall(index) => {
                        if let Some(sh_cell) = spatial_hash.get(*coord) {
                            index + (sh_cell.wall_neighbours.bitmap() as u32)
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                continue;
            };

            if let Some(instance_slot) = mapper_iter_mut.next() {
                *instance_slot = Instance {
                    sprite_index: sprite_index as f32,
                    size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
                    coord: Vector2::new(position.x * (input_sprite::WIDTH_PX as f32),
                                        position.y * (input_sprite::HEIGHT_PX as f32)).into(),
                    depth,
                };

                count += 1;
            } else {
                break;
            }
        }

        self.bundle.slice.instances = Some((count, 0));
        self.num_instances = count as usize;
    }
}
