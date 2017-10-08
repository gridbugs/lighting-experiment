use std::time::Duration;
use gfx;

use cgmath::{Vector2, ElementWise};

use renderer::sprite_sheet::{SpriteSheetTexture, TileSpriteTable, SpriteResolution};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::instance_manager::InstanceManager;
use renderer::render_target::RenderTarget;
use renderer::common;
use renderer::sizes;
use renderer::dimensions::{Dimensions, FixedDimensions, OutputDimensions, WorldDimensions};
use renderer::vision_buffer::VisionBuffer;
use renderer::frame_info::{FrameInfo, FrameInfoBuffer};
use renderer::template;
use renderer::scroll_offset::{ScrollOffset, ScrollOffsetBuffer};

use direction::{Direction, DirectionBitmap};
use content::{TileSprite, DepthType, DepthInfo};
use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;
use vision::VisionGrid;

use frontend::{OutputWorldState, LightUpdate, VisibleRange};
use res::input_sprite;
use util::time::duration_millis;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    sprite_sheet_pix_coord: [f32; 2] = "a_SpriteSheetPixCoord",
    position: [f32; 2] = "a_Position",
    pix_size: [f32; 2] = "a_PixSize",
    pix_offset: [f32; 2] = "a_PixOffset",
    depth: f32 = "a_Depth",
    depth_type: u32 = "a_DepthType",
    flags: u32 = "a_Flags",
    sprite_effect: u32 = "a_SpriteEffect",
    sprite_effect_args: [f32; 4] = "a_SpriteEffectArgs",
    hide_in_dark: u32 = "a_HideInDark",
});

gfx_constant_struct!( Light {
    colour: [f32; 4] = "colour",
    position: [f32; 4] = "position",
});

gfx_pipeline!( pipe {
    vision_table: gfx::ShaderResource<u8> = "t_VisionTable",
    light_table: gfx::ShaderResource<u8> = "t_LightTable",
    light_list: gfx::ConstantBuffer<Light> = "LightList",
    fixed_dimensions: gfx::ConstantBuffer<FixedDimensions> = "FixedDimensions",
    output_dimensions: gfx::ConstantBuffer<OutputDimensions> = "OutputDimensions",
    world_dimensions: gfx::ConstantBuffer<WorldDimensions> = "WorldDimensions",
    scroll_offset: gfx::ConstantBuffer<ScrollOffset> = "ScrollOffset",
    frame_info: gfx::ConstantBuffer<FrameInfo> = "FrameInfo",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub mod instance_flags {
    pub const ENABLED: u32 = 1 << 0;
    pub const SPRITE_EFFECT: u32 = 1 << 1;
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            // the sprite sheet ensures there's a blank sprite here
            sprite_sheet_pix_coord: [0.0, 0.0],
            position: [0.0, 0.0],
            pix_size: [input_sprite::WIDTH_PX as f32,
                       input_sprite::HEIGHT_PX as f32],
            pix_offset: [0.0, 0.0],
            depth: -1.0,
            depth_type: 0,
            flags: 0,
            sprite_effect: 0,
            sprite_effect_args: [0.0, 0.0, 0.0, 0.0],
            hide_in_dark: 0,
        }
    }
}

impl Instance {
    pub fn update_sprite_info(&mut self, sprite_info: SpriteRenderInfo) {
        let SpriteRenderInfo { position, size, offset, .. } = sprite_info;
        self.sprite_sheet_pix_coord = position;
        self.pix_size = size;
        self.pix_offset = offset;
    }

    pub fn update_depth(&mut self, y_position: f32, max_y_position: f32, depth: DepthInfo) {

        self.depth_type = depth.typ as u32;

        match depth.typ {
            DepthType::Fixed | DepthType::Gradient => {
                let mut y_position_with_offset = y_position + depth.offset;
                if y_position_with_offset > max_y_position {
                    y_position_with_offset = max_y_position;
                } else if y_position_with_offset < 0.0 {
                    y_position_with_offset = 0.0;
                }
                self.depth = y_position_with_offset;
            }
            DepthType::Bottom => {
                self.depth = depth.offset;
            }
        }
    }
}

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    instance_upload: gfx::handle::Buffer<R, Instance>,
    vision_upload: gfx::handle::Buffer<R, u8>,
    light_upload: gfx::handle::Buffer<R, u8>,
    light_list_upload: gfx::handle::Buffer<R, Light>,
    vision_buffer: gfx::handle::Buffer<R, u8>,
    light_buffer: gfx::handle::Buffer<R, u8>,
    sprite_table: TileSpriteTable,
    num_instances: usize,
    num_cells: usize,
    instance_manager: InstanceManager,
    mid_position: Vector2<f32>,
    world_width: u32,
    world_height: u32,
    visible_range: VisibleRange,
}

pub struct SpriteRenderInfo {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub offset: [f32; 2],
    pub wall_info: Option<WallSpriteRenderInfo>,
}

#[derive(Clone, Copy, Debug)]
pub struct WallSpriteRenderInfo {
    base_x: f32,
    size: f32,
}

impl WallSpriteRenderInfo {
    pub fn resolve(sprite: TileSprite, sprite_table: &TileSpriteTable) -> Option<Self> {
        if let Some(&SpriteResolution::Wall(location)) = sprite_table.get(sprite) {
            return Some(Self {
                base_x: location.base(),
                size: location.size().x,
            });
        }
        None
    }
    pub fn position(self, bitmap: u8) -> Vector2<f32> {
        Vector2::new(self.base_x + self.size * bitmap as f32, 0.0)
    }
}

impl SpriteRenderInfo {
    pub fn resolve(sprite: TileSprite, sprite_table: &TileSpriteTable,
               position: Vector2<f32>, spatial_hash: &SpatialHashTable) -> Option<Self> {
        if let Some(sprite_resolution) = sprite_table.get(sprite) {
            let (position_x, size, offset, wall_info) = match sprite_resolution {
                &SpriteResolution::Simple(location) => {
                    (location.position, location.size, location.offset, None)
                }
                &SpriteResolution::Wall(location) => {
                    if let Some(sh_cell) = spatial_hash.get_float(position) {
                        let bitmap = sh_cell.wall_neighbours.bitmap_raw();
                        (location.position(bitmap), *location.size(), *location.offset(), Some(WallSpriteRenderInfo {
                            base_x: location.base(),
                            size: location.size().x,
                        }))
                    } else {
                        return None;
                    }
                }
                &SpriteResolution::WallFit { ref top, ref front } => {
                    if let Some(sh_cell) = spatial_hash.get_float(position) {
                        let location = if sh_cell.wall_neighbours.has(Direction::North) {
                            top
                        } else {
                            front
                        };
                        (location.position, location.size, location.offset, None)
                    } else {
                        return None;
                    }
                }
            };

            return Some(Self {
                position: [position_x, 0.0],
                size: size.into(),
                offset: offset.into(),
                wall_info,
            });
        }

        None
    }

    pub fn blank() -> Self {
        Self {
            position: [0.0, 0.0],
            size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
            offset: [0.0, 0.0],
            wall_info: None,
        }
    }
}

impl<R: gfx::Resources> TileRenderer<R> {
    pub fn new<F>(sprite_sheet: &SpriteSheetTexture<R>,
                  sprite_table: TileSpriteTable,
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
            template::populate_shader(&handlebars, &context, include_bytes!("shaders/tile_renderer.150.hbs.vert")).as_bytes(),
            template::populate_shader(&handlebars, &context, include_bytes!("shaders/tile_renderer.150.hbs.frag")).as_bytes(),
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

        let light_buffer = common::create_transfer_dst_buffer(sizes::LIGHT_BUFFER_SIZE, factory)
            .expect("Failed to create light buffer");

        let light_buffer_srv = factory.view_buffer_as_shader_resource(&light_buffer)
            .expect("Failed to view light buffer as shader resource");

        let light_list = common::create_transfer_dst_buffer(sizes::MAX_NUM_LIGHTS, factory)
            .expect("Failed to create light list");

        let data = pipe::Data {
            vision_table: vision_buffer.srv.clone(),
            light_table: light_buffer_srv,
            light_list,
            fixed_dimensions: dimensions.fixed_dimensions.clone(),
            output_dimensions: dimensions.output_dimensions.clone(),
            world_dimensions: dimensions.world_dimensions.clone(),
            scroll_offset: scroll_offset_buffer.clone(),
            frame_info: frame_info_buffer.clone(),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(sizes::MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            out_colour: target.rtv.clone(),
            out_depth: target.dsv.clone(),
            tex: (sprite_sheet.srv.clone(), sampler),
        };

        let ret = Self {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            instance_upload: factory.create_upload_buffer(sizes::MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            vision_upload: factory.create_upload_buffer(sizes::TBO_VISION_BUFFER_SIZE)
                .expect("Failed to create upload buffer"),
            light_upload: factory.create_upload_buffer(sizes::LIGHT_BUFFER_SIZE)
                .expect("Failed to create upload buffer"),
            light_list_upload: factory.create_upload_buffer(sizes::MAX_NUM_LIGHTS)
                .expect("Failed to create upload buffer"),
            vision_buffer: vision_buffer.buffer.clone(),
            light_buffer,
            sprite_table,
            num_instances: 0,
            num_cells: 0,
            instance_manager: InstanceManager::new(),
            mid_position: Vector2::new(0.0, 0.0),
            world_width: 0,
            world_height: 0,
            visible_range: VisibleRange::default(),
        };
        ret
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
        encoder.copy_buffer(&self.instance_upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instances");
        encoder.copy_buffer(&self.vision_upload, &self.vision_buffer, 0, 0, self.num_cells * sizes::TBO_VISION_ENTRY_SIZE)
            .expect("Failed to copy cells");
        encoder.copy_buffer(&self.light_upload, &self.light_buffer, 0, 0, sizes::LIGHT_BUFFER_SIZE)
            .expect("Failed to copy light info");
        encoder.copy_buffer(&self.light_list_upload, &self.bundle.data.light_list, 0, 0, sizes::MAX_NUM_LIGHTS)
            .expect("Failed to copy light info");
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }

    pub fn world_state<F>(&mut self, target: &RenderTarget<R>, factory: &mut F) -> RendererWorldState<R>
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let instance_writer = factory.write_mapping(&self.instance_upload)
            .expect("Failed to map upload buffer");

        let vision_writer = factory.write_mapping(&self.vision_upload)
            .expect("Failed to map upload buffer");

        let light_grid_writer = factory.write_mapping(&self.light_upload)
            .expect("Failed to map upload buffer");

        let light_writer = factory.write_mapping(&self.light_list_upload)
            .expect("Failed to map upload buffer");

        RendererWorldState {
            instance_writer,
            vision_writer,
            light_grid_writer,
            light_writer,
            world_width: self.world_width,
            bundle: &mut self.bundle,
            sprite_table: &self.sprite_table,
            instance_manager: &mut self.instance_manager,
            num_instances: &mut self.num_instances,
            player_position: None,
            width_px: target.width,
            height_px: target.height,
            mid_position: &mut self.mid_position,
            frame_count: 0,
            total_time_ms: 0,
            next_light_index: 0,
            visible_range: &mut self.visible_range,
            num_rows: target.num_rows,
        }
    }

    pub fn handle_resize<C>(&mut self, target: &RenderTarget<R>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.bundle.data.out_colour = target.rtv.clone();
        self.bundle.data.out_depth = target.dsv.clone();
        self.visible_range = compute_visible_range(self.mid_position, target.width as i32, target.num_rows);

        let scroll_offset = compute_scroll_offset(target.width, target.height, self.mid_position);
        encoder.update_constant_buffer(&self.bundle.data.scroll_offset, &ScrollOffset {
            scroll_offset_pix: scroll_offset.into(),
        });
    }

    pub fn update_world_size(&mut self, width: u32, height: u32) {
        let num_cells = (width * height) as usize;

        if num_cells > sizes::MAX_CELL_TABLE_SIZE {
            panic!("World too big for shader");
        }

        self.num_cells = num_cells;
        self.world_width = width;
        self.world_height = height;
    }

    pub fn visible_range(&self) -> VisibleRange {
        self.visible_range
    }
}

pub struct RendererWorldState<'a, R: gfx::Resources> {
    instance_writer: gfx::mapping::Writer<'a, R, Instance>,
    vision_writer: gfx::mapping::Writer<'a, R, u8>,
    light_grid_writer: gfx::mapping::Writer<'a, R, u8>,
    light_writer: gfx::mapping::Writer<'a, R, Light>,
    world_width: u32,
    bundle: &'a mut gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    sprite_table: &'a TileSpriteTable,
    instance_manager: &'a mut InstanceManager,
    num_instances: &'a mut usize,
    player_position: Option<Vector2<f32>>,
    mid_position: &'a mut Vector2<f32>,
    width_px: u16,
    height_px: u16,
    frame_count: u64,
    total_time_ms: u64,
    next_light_index: usize,
    visible_range: &'a mut VisibleRange,
    num_rows: u16,
}

impl<'a, 'b, R: gfx::Resources> OutputWorldState<'a, 'b> for RendererWorldState<'a, R> {

    type VisionCellGrid = TboVisionGrid<'b>;
    type LightCellGrid = TboVisionGrid<'b>;
    type LightUpdate = Light;

    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {
        self.instance_manager.update(&mut self.instance_writer, change, entity_store, spatial_hash, self.sprite_table);
    }

    fn set_player_position(&mut self, player_position: Vector2<f32>) {
        self.player_position = Some(player_position);
    }

    fn set_frame_info(&mut self, frame_count: u64, total_time: Duration) {
        self.frame_count = frame_count;
        self.total_time_ms = duration_millis(total_time);
    }

    fn vision_grid(&'b mut self) -> Self::VisionCellGrid {
        TboVisionGrid {
            slice: &mut self.vision_writer,
            width: self.world_width,
        }
    }

    fn next_light(&'b mut self) -> Option<(Self::LightCellGrid, &'b mut Self::LightUpdate)> {
        if self.next_light_index < sizes::MAX_NUM_LIGHTS {
            let index = self.next_light_index;
            self.next_light_index += 1;

            let start = index * sizes::TBO_VISION_BUFFER_SIZE;
            let end = start + sizes::TBO_VISION_BUFFER_SIZE;

            Some((TboVisionGrid {
                slice: &mut self.light_grid_writer[start..end],
                width: self.world_width,
            }, &mut self.light_writer[index]))
        } else {
            None
        }
    }
}

impl<'a, R: gfx::Resources> RendererWorldState<'a, R> {
    pub fn finalise<C>(self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        let num_instances = self.instance_manager.num_instances();
        *self.num_instances = num_instances as usize;
        self.bundle.slice.instances = Some((num_instances, 0));

        if let Some(player_position) = self.player_position {
            *self.mid_position = player_position;
            let scroll_offset = compute_scroll_offset(self.width_px, self.height_px, player_position);
            *self.visible_range = compute_visible_range(player_position, self.width_px as i32, self.num_rows);
            encoder.update_constant_buffer(&self.bundle.data.scroll_offset, &ScrollOffset {
                scroll_offset_pix: scroll_offset.into(),
            });
        }
        FrameInfo::update(&self.bundle.data.frame_info, self.frame_count, self.total_time_ms, self.next_light_index, encoder);
    }
}

fn compute_scroll_offset(width: u16, height: u16, mid_position: Vector2<f32>) -> Vector2<f32> {
    let mid = (mid_position + Vector2::new(0.5, 0.5))
        .mul_element_wise(Vector2::new(input_sprite::WIDTH_PX, input_sprite::HEIGHT_PX).cast());
    Vector2::new(mid.x - (width / 2) as f32, mid.y - (height / 2) as f32)
}

fn compute_visible_range(mid_position: Vector2<f32>, width_px: i32, num_rows: u16) -> VisibleRange {
    let dy = (num_rows / 2) as i32;
    let mid_y = mid_position.y as i32; // rounded down

    let y_min = mid_y - dy;

    // if we're between cells, add 1
    let y_max = mid_y + dy + (mid_position.y != 0.0) as i32;

    let mid_x = mid_position.x as i32;
    // pad dx unless there's an integer number of cells
    let double_cell_width = input_sprite::WIDTH_PX as i32 * 2;
    let dx = width_px / double_cell_width + (width_px % double_cell_width != 0) as i32;

    let x_min = mid_x - dx;

    // if we're between cells, add 1
    let x_max = mid_x + dx + (mid_position.x != 0.0) as i32;

    VisibleRange { x_min, x_max, y_min, y_max }
}

struct TboVisionCell<'a>(&'a mut [u8]);

impl<'a> TboVisionCell<'a> {
    fn see(&mut self, bitmap: DirectionBitmap, mut time: u64) {
        for i in 0..sizes::TBO_VISION_FRAME_COUNT_SIZE {
            self.0[i] = time as u8;
            time >>= 8;
        }
        self.0[sizes::TBO_VISION_BITMAP_OFFSET] = bitmap.raw;
    }
}

pub struct TboVisionGrid<'a> {
    slice: &'a mut [u8],
    width: u32,
}

impl<'a> VisionGrid for TboVisionGrid<'a> {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64) {
        let index = ((v.y * self.width + v.x) as usize) * sizes::TBO_VISION_ENTRY_SIZE;
        TboVisionCell(&mut self.slice[index..index + sizes::TBO_VISION_ENTRY_SIZE]).see(bitmap, time);
    }
}

impl LightUpdate for Light {
    fn set_position(&mut self, position: Vector2<f32>) {
        self.position[0] = position.x;
        self.position[1] = position.y;
    }
    fn set_height(&mut self, height: f32) {
        self.position[2] = height;
    }
    fn set_colour(&mut self, colour: [f32; 3]) {
        self.colour[0] = colour[0];
        self.colour[1] = colour[1];
        self.colour[2] = colour[2];
    }
    fn set_intensity(&mut self, intensity: f32) {
        self.colour[3] = intensity;
    }
}
