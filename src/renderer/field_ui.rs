use std::cmp;
use gfx;
use cgmath::Vector2;

use renderer::formats::{ColourFormat, DepthFormat};
use renderer::render_target::RenderTarget;
use renderer::sprite_sheet::{FieldUiSpriteTable, SpriteSheetTexture, SpriteLocation};
use renderer::vision_buffer::VisionBuffer;
use renderer::frame_info::{FrameInfo, FrameInfoBuffer};
use renderer::scroll_offset::{ScrollOffset, ScrollOffsetBuffer};

use renderer::dimensions::{Dimensions, FixedDimensions, OutputDimensions, WorldDimensions};
use renderer::common;
use renderer::template;
use renderer::sizes;

use entity_store::EntityStore;
use content::{HealthInfo, FieldUiSprite};
use res::input_sprite;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    sprite_sheet_pix_coord: [f32; 2] = "a_SpriteSheetPixCoord",
    position: [f32; 2] = "a_Position",
    pix_size: [f32; 2] = "a_PixSize",
    pix_offset: [f32; 2] = "a_PixOffset",
    depth: f32 = "a_Depth",
});

gfx_pipeline!( pipe {
    scroll_offset: gfx::ConstantBuffer<ScrollOffset> = "ScrollOffset",
    frame_info: gfx::ConstantBuffer<FrameInfo> = "FrameInfo",
    vision_table: gfx::ShaderResource<u8> = "t_VisionTable",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    fixed_dimensions: gfx::ConstantBuffer<FixedDimensions> = "FixedDimensions",
    output_dimensions: gfx::ConstantBuffer<OutputDimensions> = "OutputDimensions",
    world_dimensions: gfx::ConstantBuffer<WorldDimensions> = "WorldDimensions",
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
});

impl Instance {
    fn from_location(sprite_location: &SpriteLocation,
                     position: Vector2<f32>,
                     offset: Vector2<i32>,
                     depth: f32) -> Self
    {
        Self {
            sprite_sheet_pix_coord: [sprite_location.position, 0.0],
            position: position.into(),
            pix_size: sprite_location.size.into(),
            pix_offset: offset.cast().into(),
            depth,
        }
    }
}

#[derive(Debug)]
struct SpriteCache {
    full_health: SpriteLocation,
    empty_health: SpriteLocation,
    health_step: Vector2<i32>,
    health_sprites_per_row: u32,
}

impl SpriteCache {
    fn new(sprite_table: &FieldUiSpriteTable) -> Self {
        let full_health = *sprite_table.get(FieldUiSprite::HealthFull).expect("Missing sprite");
        let health_step = Vector2::new(full_health.size.x as i32 + 1, full_health.size.y as i32 + 1);
        let health_sprites_per_row = input_sprite::WIDTH_PX / (health_step.x as u32);
        Self {
            full_health,
            empty_health: *sprite_table.get(FieldUiSprite::HealthEmpty).expect("Missing sprite"),
            health_step,
            health_sprites_per_row,
        }
    }
}

pub struct FieldUi<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    instance_upload: gfx::handle::Buffer<R, Instance>,
    sprite_table: FieldUiSpriteTable,
    sprite_cache: SpriteCache,
}

impl<R: gfx::Resources> FieldUi<R> {
    pub fn new<F>(sprite_sheet: &SpriteSheetTexture<R>,
                  sprite_table: FieldUiSpriteTable,
                  target: &RenderTarget<R>,
                  dimensions: &Dimensions<R>,
                  vision_buffer: &VisionBuffer<R>,
                  frame_info_buffer: &FrameInfoBuffer<R>,
                  scroll_offset_buffer: &ScrollOffsetBuffer<R>,
                  factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let (handlebars, context) = template::make_shader_template_context();

        let pso = factory.create_pipeline_simple(
            template::populate_shader(&handlebars, &context, include_bytes!("shaders/field_ui.150.hbs.vert")).as_bytes(),
            template::populate_shader(&handlebars, &context, include_bytes!("shaders/field_ui.150.hbs.frag")).as_bytes(),
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

        let data = pipe::Data {
            scroll_offset: scroll_offset_buffer.clone(),
            frame_info: frame_info_buffer.clone(),
            vision_table: vision_buffer.srv.clone(),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(sizes::FIELD_UI_MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            fixed_dimensions: dimensions.fixed_dimensions.clone(),
            output_dimensions: dimensions.output_dimensions.clone(),
            world_dimensions: dimensions.world_dimensions.clone(),
            out_colour: target.rtv.clone(),
            out_depth: target.dsv.clone(),
            tex: (sprite_sheet.srv.clone(), sampler),
        };

        let bundle = gfx::pso::bundle::Bundle::new(slice, pso, data);

        let sprite_cache = SpriteCache::new(&sprite_table);

        Self {
            bundle,
            instance_upload: factory.create_upload_buffer(sizes::FIELD_UI_MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            sprite_table,
            sprite_cache,
        }
    }

    pub fn handle_resize<C>(&mut self, target: &RenderTarget<R>, _encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.bundle.data.out_colour = target.rtv.clone();
        self.bundle.data.out_depth = target.dsv.clone();
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.clear(&self.bundle.data.out_colour, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    }

    pub fn draw<C, F>(&mut self, entity_store: &EntityStore, encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let num_instances = {
            let mut writer = factory.write_mapping(&self.instance_upload)
                .expect("Failed to map upload buffer");

            Self::draw_health(&self.sprite_cache, &mut writer, 0, entity_store)
        };

        self.bundle.slice.instances = Some((num_instances, 0));

        encoder.copy_buffer(&self.instance_upload, &self.bundle.data.instance, 0, 0, num_instances as usize)
            .expect("Failed to copy instance buffer");
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }

    fn draw_health(sprite_cache: &SpriteCache,
                   instances: &mut [Instance],
                   base: u32,
                   entity_store: &EntityStore) -> u32
    {
        let mut count = base;
        for (id, offsets) in entity_store.field_ui.iter() {
            let position = if let Some(position) = entity_store.position.get(id) {
                *position
            } else {
                continue;
            };

            if let Some(health) = entity_store.health.get(id) {
                count = Self::draw_health_entity(sprite_cache, offsets.health_vertical, instances, count, position, *health);
            }
        }

        count
    }

    fn draw_health_entity(sprite_cache: &SpriteCache,
                          vertical_offset: i32,
                          instances: &mut [Instance],
                          mut base: u32,
                          position: Vector2<f32>,
                          health: HealthInfo) -> u32
    {   let health_current = cmp::max(health.current, 0) as u32;
        let base_offset = Vector2::new(0, vertical_offset);

        for i in 0..cmp::max(health.max, 0) as u32 {
            let sprite = if i < health_current {
                &sprite_cache.full_health
            } else {
                &sprite_cache.empty_health
            };

            let row = (i / sprite_cache.health_sprites_per_row) as i32;
            let col = (i % sprite_cache.health_sprites_per_row) as i32;

            let offset = base_offset + Vector2::new(-col * sprite_cache.health_step.x, row * sprite_cache.health_step.y);

            instances[base as usize] = Instance::from_location(sprite, position, offset, 0.0);
            base += 1;
        }

        base
    }
}
