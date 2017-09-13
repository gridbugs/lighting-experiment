use std::time::Duration;
use gfx;

use cgmath::{Vector2, ElementWise};
use handlebars::Handlebars;

use renderer::sprite_sheet::{SpriteSheet, SpriteTable, SpriteResolution};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::instance_manager::InstanceManager;
use renderer::common;

use direction::{Direction, NO_DIRECTIONS_BITMAP, DirectionBitmap};
use content::{Sprite, DepthType, DepthInfo, SpriteEffect};
use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;
use vision::{VisionGrid, VisionGridWithHistory};

use frontend::{OutputWorldState, LightUpdate};
use res::input_sprite;
use util::time::duration_millis;

const NUM_ROWS: u16 = 15;
const HEIGHT_PX: u16 = NUM_ROWS * input_sprite::HEIGHT_PX as u16;
const MAX_NUM_INSTANCES: usize = 4096;
const MAX_CELL_TABLE_SIZE: usize = 4096;
const MAX_NUM_LIGHTS: usize = 32;
const LIGHT_BUFFER_NUM_ENTRIES: usize = MAX_NUM_LIGHTS * MAX_CELL_TABLE_SIZE;
const LIGHT_BUFFER_ENTRY_SIZE_FRAME_COUNT: usize = 5; // 40 bit uint
const LIGHT_BUFFER_ENTRY_SIZE_SIDE_BITMAP: usize = 1; // 8 bit bitmap
const LIGHT_BUFFER_ENTRY_SIZE: usize = LIGHT_BUFFER_ENTRY_SIZE_FRAME_COUNT +
                                       LIGHT_BUFFER_ENTRY_SIZE_SIDE_BITMAP;
const LIGHT_BUFFER_SIZE: usize = LIGHT_BUFFER_NUM_ENTRIES * LIGHT_BUFFER_ENTRY_SIZE;
const LIGHT_BUFFER_SIZE_PER_LIGHT: usize = MAX_CELL_TABLE_SIZE * LIGHT_BUFFER_ENTRY_SIZE;
const LIGHT_BUFFER_OFFSET_SIDE_BITMAP: usize = LIGHT_BUFFER_ENTRY_SIZE_FRAME_COUNT;

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
});

gfx_constant_struct!( FixedDimensions {
    sprite_sheet_size: [f32; 2] = "u_SpriteSheetSize",
    cell_size: [f32; 2] = "u_CellSize",
});

gfx_constant_struct!( OutputDimensions {
    output_size: [f32; 2] = "u_OutputSize",
});

gfx_constant_struct!( WorldDimensions {
    world_size: [f32; 2] = "u_WorldSize",
    world_size_u32: [u32; 2] = "u_WorldSizeUint",
});

gfx_constant_struct!( Offset {
    scroll_offset_pix: [f32; 2] = "u_ScrollOffsetPix",
});

gfx_constant_struct!( FrameInfo {
    frame_count: [u32; 2] = "u_FrameCount_u64",
    total_time_ms: [u32; 2] = "u_TotalTimeMs_u64",
    num_lights: u32 = "u_NumLights",
});

gfx_constant_struct!( Cell {
    last: [u32; 2] = "last_u64",
    current_side_bitmap: u32 = "current_side_bitmap",
    history_side_bitmap: u32 = "history_side_bitmap",
});

gfx_constant_struct!( Light {
    colour: [f32; 3] = "colour",
    _pad0: u32 = "_pad0",
    position: [f32; 3] = "position",
    intensity: f32 = "intensity",
});

gfx_pipeline!( pipe {
    vision_table: gfx::ConstantBuffer<Cell> = "VisionTable",
    light_table: gfx::ShaderResource<u8> = "t_LightTable",
    light_list: gfx::ConstantBuffer<Light> = "LightList",
    fixed_dimensions: gfx::ConstantBuffer<FixedDimensions> = "FixedDimensions",
    output_dimensions: gfx::ConstantBuffer<OutputDimensions> = "OutputDimensions",
    world_dimensions: gfx::ConstantBuffer<WorldDimensions> = "WorldDimensions",
    offset: gfx::ConstantBuffer<Offset> = "Offset",
    frame_info: gfx::ConstantBuffer<FrameInfo> = "FrameInfo",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

impl Default for Cell {
    fn default() -> Self {
        Self {
            last: [0; 2],
            current_side_bitmap: NO_DIRECTIONS_BITMAP as u32,
            history_side_bitmap: NO_DIRECTIONS_BITMAP as u32,
        }
    }
}

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

fn u64_to_arr(u: u64) -> [u32; 2] {
    [ u as u32, (u >> 32) as u32 ]
}

impl FrameInfo {
    fn set_time(&mut self, time: u64) {
        self.frame_count = u64_to_arr(time);
    }
}

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    instance_upload: gfx::handle::Buffer<R, Instance>,
    vision_upload: gfx::handle::Buffer<R, Cell>,
    light_upload: gfx::handle::Buffer<R, u8>,
    light_list_upload: gfx::handle::Buffer<R, Light>,
    light_buffer: gfx::handle::Buffer<R, u8>,
    sprite_sheet: SpriteSheet<R>,
    width_px: u16,
    height_px: u16,
    num_instances: usize,
    num_cells: usize,
    instance_manager: InstanceManager,
    mid_position: Vector2<f32>,
    world_width: u32,
    world_height: u32,
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
    pub fn resolve(sprite: Sprite, sprite_table: &SpriteTable) -> Option<Self> {
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
    pub fn resolve(sprite: Sprite, sprite_table: &SpriteTable,
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

fn populate_shader(shader: &[u8]) -> String {
    let handlebars = {
        let mut h = Handlebars::new();
        h.register_escape_fn(|input| input.to_string());
        h
    };

    let table = hashmap!{
        "FLAGS_ENABLED" => instance_flags::ENABLED,
        "FLAGS_SPRITE_EFFECT" => instance_flags::SPRITE_EFFECT,
        "DEPTH_FIXED" => DepthType::Fixed as u32,
        "DEPTH_GRADIENT" => DepthType::Gradient as u32,
        "DEPTH_BOTTOM" => DepthType::Bottom as u32,
        "MAX_CELL_TABLE_SIZE" => MAX_CELL_TABLE_SIZE as u32,
        "SPRITE_EFFECT_OUTER_WATER" => SpriteEffect::OuterWater as u32,
        "MAX_NUM_LIGHTS" => MAX_NUM_LIGHTS as u32,
        "LIGHT_BUFFER_ENTRY_SIZE" => LIGHT_BUFFER_ENTRY_SIZE as u32,
        "LIGHT_BUFFER_OFFSET_SIDE_BITMAP" => LIGHT_BUFFER_OFFSET_SIDE_BITMAP as u32,
        "LIGHT_BUFFER_SIZE_PER_LIGHT" => LIGHT_BUFFER_SIZE_PER_LIGHT as u32,
    };

    let shader_str = ::std::str::from_utf8(shader)
        .expect("Failed to convert shader to utf8");

    handlebars.template_render(shader_str, &table)
        .expect("Failed to render shader template")
}

impl<R: gfx::Resources> TileRenderer<R> {
    fn scaled_width(window_width_px: u16, window_height_px: u16) -> u16 {
        ((window_width_px as u32 * HEIGHT_PX as u32) / window_height_px as u32) as u16
    }
    fn create_targets<F>(window_width_px: u16, window_height_px: u16, factory: &mut F)
        -> (u16, gfx::handle::ShaderResourceView<R, [f32; 4]>,
            gfx::handle::RenderTargetView<R, ColourFormat>,
            gfx::handle::DepthStencilView<R, DepthFormat>)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let width_px = Self::scaled_width(window_width_px, window_height_px);
        let (_, srv, colour_rtv) = factory.create_render_target(width_px, HEIGHT_PX)
            .expect("Failed to create render target for sprite sheet");
        let (_, _, depth_rtv) = factory.create_depth_stencil(width_px, HEIGHT_PX)
            .expect("Failed to create depth stencil");
        (width_px, srv, colour_rtv, depth_rtv)
    }
    pub fn new<F>(sprite_sheet: SpriteSheet<R>,
                  window_width_px: u16,
                  window_height_px: u16,
                  factory: &mut F) -> (Self, gfx::handle::ShaderResourceView<R, [f32; 4]>)
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            populate_shader(include_bytes!("shaders/tile_renderer.150.hbs.vert")).as_bytes(),
            populate_shader(include_bytes!("shaders/tile_renderer.150.hbs.frag")).as_bytes(),
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

        let (width_px, srv, colour_rtv, depth_rtv) =
            Self::create_targets(window_width_px, window_height_px, factory);

        let vision_table = common::create_transfer_dst_buffer(MAX_CELL_TABLE_SIZE, factory)
            .expect("Failed to create cell table");

        let light_buffer = common::create_transfer_dst_buffer(LIGHT_BUFFER_SIZE, factory)
            .expect("Failed to create cell table");

        let light_buffer_srv = factory.view_buffer_as_shader_resource(&light_buffer)
            .expect("Failed to view light buffer as shader resource");

        let light_list = common::create_transfer_dst_buffer(MAX_NUM_LIGHTS, factory)
            .expect("Failed to create light list");

        let data = pipe::Data {
            vision_table,
            light_table: light_buffer_srv,
            light_list,
            fixed_dimensions: factory.create_constant_buffer(1),
            output_dimensions: factory.create_constant_buffer(1),
            world_dimensions: factory.create_constant_buffer(1),
            offset: factory.create_constant_buffer(1),
            frame_info: factory.create_constant_buffer(1),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            out_colour: colour_rtv,
            out_depth: depth_rtv,
            tex: (sprite_sheet.shader_resource_view.clone(), sampler),
        };

        (TileRenderer {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            instance_upload: factory.create_upload_buffer(MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            vision_upload: factory.create_upload_buffer(MAX_CELL_TABLE_SIZE)
                .expect("Failed to create upload buffer"),
            light_upload: factory.create_upload_buffer(LIGHT_BUFFER_SIZE)
                .expect("Failed to create upload buffer"),
            light_list_upload: factory.create_upload_buffer(MAX_NUM_LIGHTS)
                .expect("Failed to create upload buffer"),
            light_buffer,
            sprite_sheet,
            width_px,
            height_px: HEIGHT_PX,
            num_instances: 0,
            num_cells: 0,
            instance_manager: InstanceManager::new(),
            mid_position: Vector2::new(0.0, 0.0),
            world_width: 0,
            world_height: 0,
        }, srv)
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width_px, self.height_px)
    }

    pub fn init<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.bundle.data.fixed_dimensions, &FixedDimensions {
            sprite_sheet_size: [self.sprite_sheet.width as f32, self.sprite_sheet.height as f32],
            cell_size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
        });
        self.update_output_dimensions(encoder);
    }

    pub fn update_output_dimensions<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.bundle.data.output_dimensions, &OutputDimensions {
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
        encoder.copy_buffer(&self.instance_upload, &self.bundle.data.instance, 0, 0, self.num_instances)
            .expect("Failed to copy instances");
        encoder.copy_buffer(&self.vision_upload, &self.bundle.data.vision_table, 0, 0, self.num_cells)
            .expect("Failed to copy cells");
        encoder.copy_buffer(&self.light_upload, &self.light_buffer, 0, 0, LIGHT_BUFFER_NUM_ENTRIES)
            .expect("Failed to copy light info");
        encoder.copy_buffer(&self.light_list_upload, &self.bundle.data.light_list, 0, 0, MAX_NUM_LIGHTS)
            .expect("Failed to copy light info");
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }

    pub fn world_state<F>(&mut self, factory: &mut F) -> RendererWorldState<R>
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
            sprite_table: &self.sprite_sheet.sprite_table,
            instance_manager: &mut self.instance_manager,
            num_instances: &mut self.num_instances,
            player_position: None,
            width_px: self.width_px,
            height_px: self.height_px,
            mid_position: &mut self.mid_position,
            frame_count: 0,
            total_time_ms: 0,
            next_light_index: 0,
        }
    }

    pub fn handle_resize<C, F>(&mut self, width: u16, height: u16,
                               encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        -> gfx::handle::ShaderResourceView<R, [f32; 4]>
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let (target_width_px, srv, colour_rtv, depth_rtv) =
            Self::create_targets(width, height, factory);

        self.width_px = target_width_px;
        self.bundle.data.out_colour = colour_rtv;
        self.bundle.data.out_depth = depth_rtv;

        self.update_output_dimensions(encoder);

        let scroll_offset = compute_scroll_offset(self.width_px, self.height_px, self.mid_position);
        encoder.update_constant_buffer(&self.bundle.data.offset, &Offset {
            scroll_offset_pix: scroll_offset.into(),
        });

        srv
    }

    pub fn update_world_size<C>(&mut self, width: u32, height: u32,
                                encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        let num_cells = (width * height) as usize;

        if num_cells > MAX_CELL_TABLE_SIZE {
            panic!("World too big for shader");
        }

        encoder.update_constant_buffer(&self.bundle.data.world_dimensions, &WorldDimensions {
            world_size: [width as f32, height as f32],
            world_size_u32: [width, height],
        });

        self.num_cells = num_cells;
        self.world_width = width;
        self.world_height = height;
    }
}

pub struct RendererWorldState<'a, R: gfx::Resources> {
    instance_writer: gfx::mapping::Writer<'a, R, Instance>,
    vision_writer: gfx::mapping::Writer<'a, R, Cell>,
    light_grid_writer: gfx::mapping::Writer<'a, R, u8>,
    light_writer: gfx::mapping::Writer<'a, R, Light>,
    world_width: u32,
    bundle: &'a mut gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    sprite_table: &'a SpriteTable,
    instance_manager: &'a mut InstanceManager,
    num_instances: &'a mut usize,
    player_position: Option<Vector2<f32>>,
    mid_position: &'a mut Vector2<f32>,
    width_px: u16,
    height_px: u16,
    frame_count: u64,
    total_time_ms: u64,
    next_light_index: usize,
}

impl Cell {
    fn see(&mut self, bitmap: DirectionBitmap, time: u64) {
        self.current_side_bitmap = bitmap.raw as u32;
        self.last = u64_to_arr(time);
    }
    fn see_with_history(&mut self, current: DirectionBitmap, history: DirectionBitmap, time: u64) {
        self.current_side_bitmap = current.raw as u32;
        self.history_side_bitmap = history.raw as u32;
        self.last = u64_to_arr(time);
    }
}

impl<'a, 'b, R: gfx::Resources> OutputWorldState<'a, 'b> for RendererWorldState<'a, R> {

    type VisionCellGrid = VisionCellGrid<'b>;
    type LightCellGrid = LightGrid<'b>;
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
        VisionCellGrid {
            slice: &mut self.vision_writer,
            width: self.world_width,
        }
    }

    fn next_light(&'b mut self) -> Option<(Self::LightCellGrid, &'b mut Self::LightUpdate)> {
        if self.next_light_index < MAX_NUM_LIGHTS {
            let index = self.next_light_index;
            self.next_light_index += 1;

            let start = index * LIGHT_BUFFER_SIZE_PER_LIGHT;
            let end = start + LIGHT_BUFFER_SIZE_PER_LIGHT;

            Some((LightGrid {
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
            encoder.update_constant_buffer(&self.bundle.data.offset, &Offset {
                scroll_offset_pix: scroll_offset.into(),
            });
        }
        encoder.update_constant_buffer(&self.bundle.data.frame_info, &FrameInfo {
            frame_count: u64_to_arr(self.frame_count),
            total_time_ms: u64_to_arr(self.total_time_ms),
            num_lights: self.next_light_index as u32,
        });

    }
}

fn compute_scroll_offset(width: u16, height: u16, mid_position: Vector2<f32>) -> Vector2<f32> {
    let mid = (mid_position + Vector2::new(0.5, 0.5))
        .mul_element_wise(Vector2::new(input_sprite::WIDTH_PX, input_sprite::HEIGHT_PX).cast());
    Vector2::new(mid.x - (width / 2) as f32, mid.y - (height / 2) as f32)
}

pub struct VisionCellGrid<'a> {
    slice: &'a mut [Cell],
    width: u32,
}

impl<'a> VisionGrid for VisionCellGrid<'a> {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64) {
        let index = (v.y * self.width + v.x) as usize;
        self.slice[index].see(bitmap, time);
    }
}

impl<'a> VisionGridWithHistory for VisionCellGrid<'a> {
    fn see_with_history(&mut self, v: Vector2<u32>, current: DirectionBitmap, history: DirectionBitmap, time: u64) {
        let index = (v.y * self.width + v.x) as usize;
        self.slice[index].see_with_history(current, history, time);
    }
}


struct LightCell<'a>(&'a mut [u8]);

impl<'a> LightCell<'a> {
    fn see(&mut self, bitmap: DirectionBitmap, mut time: u64) {
        for i in 0..LIGHT_BUFFER_ENTRY_SIZE_FRAME_COUNT {
            self.0[i] = time as u8;
            time >>= 8;
        }
        self.0[LIGHT_BUFFER_OFFSET_SIDE_BITMAP] = bitmap.raw;
    }
}

pub struct LightGrid<'a> {
    slice: &'a mut [u8],
    width: u32,
}

impl<'a> LightGrid<'a> {
    fn get(&mut self, index: usize) -> LightCell {
        LightCell(&mut self.slice[index..index + LIGHT_BUFFER_ENTRY_SIZE])
    }
}

impl<'a> VisionGrid for LightGrid<'a> {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64) {
        let index = ((v.y * self.width + v.x) as usize) * LIGHT_BUFFER_ENTRY_SIZE;
        LightCell(&mut self.slice[index..index + LIGHT_BUFFER_ENTRY_SIZE]).see(bitmap, time);
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
        self.colour = colour;
    }
    fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }
}
