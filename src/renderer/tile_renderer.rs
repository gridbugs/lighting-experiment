use gfx;

use cgmath::{Vector2, ElementWise};

use renderer::sprite_sheet::{SpriteSheet, SpriteTable, SpriteResolution};
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::instance_manager::InstanceManager;
use renderer::common;

use direction::Direction;
use content::{Sprite, DepthType, DepthInfo};
use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;

use frontend::OutputWorldState;
use res::input_sprite;

const NUM_ROWS: u16 = 15;
const HEIGHT_PX: u16 = NUM_ROWS * input_sprite::HEIGHT_PX as u16;
const MAX_NUM_INSTANCES: usize = 4096;
const MAX_Y: f32 = 1000.0;

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
});

gfx_constant_struct!( Dimensions {
    sprite_sheet_size: [f32; 2] = "u_SpriteSheetSize",
    output_size: [f32; 2] = "u_OutputSize",
    cell_size: [f32; 2] = "u_CellSize",
    max_y: f32 = "u_MaxY",
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

pub mod depth_type {
    pub const DISABLED: u32 = 0;
    pub const FIXED: u32 = 1;
    pub const GRADIENT: u32 = 2;
    pub const BOTTOM: u32 = 3;
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            // the sprite sheet ensures there's a blank sprite here
            sprite_sheet_pix_coord: [0.0, 0.0],
            position: [0.0, 0.0],
            pix_size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
            pix_offset: [0.0, 0.0],
            depth: -1.0,
            depth_type: 0,
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

    pub fn update_depth(&mut self, y_position: f32, _max_y_position: f32, depth: DepthInfo) {

        let mut y_position_with_offset = y_position + depth.offset;
        if y_position_with_offset > MAX_Y {
            y_position_with_offset = MAX_Y;
        } else if y_position_with_offset < 0.0 {
            y_position_with_offset = 0.0;
        }
        self.depth = y_position_with_offset;

        match depth.typ {
            DepthType::YAxis => {
                self.depth_type = depth_type::FIXED;
            }
            DepthType::Bottom => {
                self.depth_type = depth_type::BOTTOM;
            }
            DepthType::Gradient => {
                self.depth_type = depth_type::GRADIENT;
            }
        }
    }
}

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    upload: gfx::handle::Buffer<R, Instance>,
    sprite_sheet: SpriteSheet<R>,
    width_px: u16,
    height_px: u16,
    num_instances: usize,
    instance_manager: InstanceManager,
    mid_position: Vector2<f32>,
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

        let (width_px, srv, colour_rtv, depth_rtv) =
            Self::create_targets(window_width_px, window_height_px, factory);

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
            instance_manager: InstanceManager::new(),
            mid_position: Vector2::new(0.0, 0.0),
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
            cell_size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
            max_y: MAX_Y,
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

    pub fn world_state<F>(&mut self, factory: &mut F) -> RendererWorldState<R>
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let writer = factory.write_mapping(&self.upload)
            .expect("Failed to map upload buffer");

        RendererWorldState {
            writer,
            bundle: &mut self.bundle,
            sprite_table: &self.sprite_sheet.sprite_table,
            instance_manager: &mut self.instance_manager,
            num_instances: &mut self.num_instances,
            player_position: None,
            width_px: self.width_px,
            height_px: self.height_px,
            mid_position: &mut self.mid_position,
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

        self.init(encoder);

        let scroll_offset = compute_scroll_offset(self.width_px, self.height_px, self.mid_position);
        encoder.update_constant_buffer(&self.bundle.data.offset, &Offset {
            scroll_offset_pix: scroll_offset.into(),
        });

        srv
    }
}

pub struct RendererWorldState<'a, R: gfx::Resources> {
    writer: gfx::mapping::Writer<'a, R, Instance>,
    bundle: &'a mut gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    sprite_table: &'a SpriteTable,
    instance_manager: &'a mut InstanceManager,
    num_instances: &'a mut usize,
    player_position: Option<Vector2<f32>>,
    mid_position: &'a mut Vector2<f32>,
    width_px: u16,
    height_px: u16,
}

impl<'a, R: gfx::Resources> OutputWorldState<'a> for RendererWorldState<'a, R> {
    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {
        self.instance_manager.update(&mut self.writer, change, entity_store, spatial_hash, self.sprite_table);
    }

    fn set_player_position(&mut self, player_position: Vector2<f32>) {
        self.player_position = Some(player_position);
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
    }
}

fn compute_scroll_offset(width: u16, height: u16, mid_position: Vector2<f32>) -> Vector2<f32> {
    let mid = (mid_position + Vector2::new(0.5, 0.5))
        .mul_element_wise(Vector2::new(input_sprite::WIDTH_PX, input_sprite::HEIGHT_PX).cast());
    Vector2::new(mid.x - (width / 2) as f32, mid.y - (height / 2) as f32)
}
