use std::mem;
use cgmath::Vector2;
use spatial_hash::SpatialHashTable;
use vision::VisionGrid;
use direction::DirectionBitmap;

use vision::shadowcast_octants::*;

#[derive(Debug, Clone, Copy)]
struct Gradient {
    lateral: i32,
    depth: i32,
}

impl Gradient {
    fn new(lateral: i32, depth: i32) -> Self {
        Self { lateral, depth }
    }
}

struct StaticParams<'a> {
    centre: Vector2<i32>,
    vision_distance_squared: i32,
    time: u64,
    spatial_hash: &'a SpatialHashTable,
}

struct ScanParams {
    min_gradient: Gradient,
    max_gradient: Gradient,
    depth: i32,
    visibility: f32,
}

impl Default for ScanParams {
    fn default() -> Self {
        Self {
            min_gradient: Gradient::new(0, 1),
            max_gradient: Gradient::new(1, 1),
            depth: 1,
            visibility: 1.0,
        }
    }
}

struct CornerInfo {
    bitmap: DirectionBitmap,
    coord: Vector2<i32>,
}

fn scan<G, O>(grid: &mut G,
              octant: &O,
              next: &mut Vec<ScanParams>,
              params: ScanParams,
              static_params: &StaticParams) -> Option<CornerInfo>
    where G: VisionGrid,
          O: Octant,
{
    let ScanParams { mut min_gradient, max_gradient, depth, visibility } = params;

    let depth_index = if let Some(depth_index) = octant.depth_index(static_params.centre, depth) {
        depth_index
    } else {
        return None;
    };

    let front_gradient_depth = depth * 2 - 1;
    let back_gradient_depth = front_gradient_depth + 2;

    let double_start_num = min_gradient.depth + front_gradient_depth * min_gradient.lateral;
    let double_stop_num = max_gradient.depth + back_gradient_depth * max_gradient.lateral;

    let lateral_min = double_start_num / (2 * min_gradient.depth);

    let stop_denom = 2 * max_gradient.depth;
    let lateral_max = if double_stop_num % stop_denom == 0 {
        (double_stop_num - 1) / stop_denom
    } else {
        double_stop_num / stop_denom
    };

    let mut first_iteration = true;
    let mut prev_visibility = 0.0;
    let mut prev_opaque = false;
    let mut lateral_index = lateral_min;

    while lateral_index <= lateral_max {
        let coord = octant.make_coord(static_params.centre, lateral_index, depth_index);
        if coord.x < 0 || coord.x >= static_params.spatial_hash.width() as i32 ||
            coord.y < 0 || coord.y >= static_params.spatial_hash.height() as i32 {
            break;
        };
        let coord_u32 = coord.cast();
        let sh_cell = if let Some(sh_cell) = static_params.spatial_hash.get(coord_u32) {
            sh_cell
        } else {
            break;
        };

        let gradient_lateral = lateral_index * 2 - 1;
        let mut direction_bitmap = DirectionBitmap::empty();

        let cur_visibility = (visibility - sh_cell.opacity_total as f32).max(0.0);
        let cur_opaque = cur_visibility == 0.0;

        if cur_opaque {
            // check if we can actually see the facing side
            if max_gradient.lateral * front_gradient_depth > gradient_lateral * max_gradient.depth {
                direction_bitmap |= octant.facing_bitmap();
            } else {
                direction_bitmap |= octant.facing_corner_bitmap();
            }
        } else {
            direction_bitmap |= DirectionBitmap::all();
        };

        // handle changes in opacity
        if !first_iteration && cur_visibility != prev_visibility {
            // use the back of the cell if necessary
            let gradient_depth = if cur_visibility < prev_visibility {
                back_gradient_depth
            } else {
                front_gradient_depth
            };
            let gradient = Gradient::new(gradient_lateral, gradient_depth);

            if !prev_opaque {
                // see beyond the previous section unless it's opaque
                next.push(ScanParams {
                    min_gradient,
                    max_gradient: gradient,
                    depth: depth + 1,
                    visibility: prev_visibility,
                });
            }

            min_gradient = gradient;
            if cur_opaque {
                // the edge of the current cell is visible through the previous cell
                direction_bitmap |= octant.across_bitmap();
            }
        }

        // check if cell is in visible range
        let between = coord - static_params.centre;
        let distance_squared = between.x * between.x + between.y * between.y;
        let visible = distance_squared < static_params.vision_distance_squared;

        // handle final cell
        if lateral_index == lateral_max {
            if !cur_opaque {
                // see beyond the current section
                next.push(ScanParams {
                    min_gradient,
                    max_gradient,
                    depth: depth + 1,
                    visibility: cur_visibility,
                });
            }
            if visible && max_gradient.lateral == max_gradient.depth {
                return Some(CornerInfo {
                    bitmap: direction_bitmap,
                    coord,
                });
            }
        }

        if visible && octant.should_see(lateral_index) {
            grid.see(coord_u32, direction_bitmap, static_params.time);
        }

        prev_visibility = cur_visibility;
        prev_opaque = cur_opaque;
        first_iteration = false;
        lateral_index += 1;
    }

    None
}

pub struct ShadowcastEnv {
    queue_a: Vec<ScanParams>,
    queue_a_swap: Vec<ScanParams>,
    queue_b: Vec<ScanParams>,
    queue_b_swap: Vec<ScanParams>,
}

impl ShadowcastEnv {
    pub fn new() -> Self {
        Self {
            queue_a: Vec::new(),
            queue_a_swap: Vec::new(),
            queue_b: Vec::new(),
            queue_b_swap: Vec::new(),
        }
    }
}

fn observe_octant<G, A, B>(grid: &mut G,
                           env: &mut ShadowcastEnv,
                           octant_a: A,
                           octant_b: B,
                           static_params: &StaticParams)
    where G: VisionGrid,
          A: Octant,
          B: Octant,
{
    env.queue_a.push(ScanParams::default());
    env.queue_b.push(ScanParams::default());

    loop {
        let mut corner_bitmap = DirectionBitmap::empty();
        let mut corner_coord = None;

        while let Some(params) = env.queue_a.pop() {
            if let Some(corner) = scan(grid, &octant_a, &mut env.queue_a_swap, params, static_params) {
                corner_bitmap |= corner.bitmap;
                corner_coord = Some(corner.coord);
            }
        }

        while let Some(params) = env.queue_b.pop() {
            if let Some(corner) = scan(grid, &octant_b, &mut env.queue_b_swap, params, static_params) {
                corner_bitmap |= corner.bitmap;
            }
        }

        if let Some(corner_coord) = corner_coord {
            grid.see(corner_coord.cast(), corner_bitmap, static_params.time);
        }

        if env.queue_a_swap.is_empty() && env.queue_b_swap.is_empty() {
            break;
        }
        mem::swap(&mut env.queue_a, &mut env.queue_a_swap);
        mem::swap(&mut env.queue_b, &mut env.queue_b_swap);
    }
}

pub fn observe<G>(grid: &mut G,
                  env: &mut ShadowcastEnv,
                  position: Vector2<f32>,
                  spatial_hash: &SpatialHashTable,
                  distance: u32,
                  time: u64)
    where G: VisionGrid,
{
    let coord = (position + Vector2::new(0.5, 0.5)).cast();

    if coord.x >= 0 && coord.y >= 0 {
        grid.see(Vector2::new(coord.x as u32, coord.y as u32), DirectionBitmap::all(), time);
    }

    let width = spatial_hash.width();
    let height = spatial_hash.height();

    let params = StaticParams {
        centre: coord,
        vision_distance_squared: (distance * distance) as i32,
        time,
        spatial_hash,
    };

    observe_octant(grid, env, TopLeft, LeftTop, &params);
    observe_octant(grid, env, TopRight { width }, RightTop { width }, &params);
    observe_octant(grid, env, BottomLeft { height }, LeftBottom { height }, &params);
    observe_octant(grid, env, BottomRight { width, height }, RightBottom { width, height }, &params);
}
