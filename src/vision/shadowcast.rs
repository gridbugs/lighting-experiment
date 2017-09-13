use std::mem;
use cgmath::Vector2;
use spatial_hash::SpatialHashTable;
use vision::VisionGrid;
use direction::DirectionBitmap;

use vision::shadowcast_octants::*;

struct StaticParams<'a> {
    centre: Vector2<i32>,
    vision_distance_squared: i32,
    time: u64,
    spatial_hash: &'a SpatialHashTable,
}

struct ScanParams {
    min_gradient: Vector2<i32>,
    max_gradient: Vector2<i32>,
    depth: i32,
    visibility: f32,
}

impl Default for ScanParams {
    fn default() -> Self {
        Self {
            min_gradient: Vector2::new(0, 1),
            max_gradient: Vector2::new(1, 1),
            depth: 1,
            visibility: 1.0,
        }
    }
}

struct CornerInfo {
    bitmap: DirectionBitmap,
    coord: Vector2<i32>,
}

fn scan<G: VisionGrid, O: Octant>(grid: &mut G,
                       octant: &O,
                       next: &mut Vec<ScanParams>,
                       params: ScanParams,
                       static_params: &StaticParams) -> Option<CornerInfo>
{
    let ScanParams { mut min_gradient, max_gradient, depth, visibility } = params;

    let y_index = if let Some(y_index) = octant.depth_index(static_params.centre, depth) {
        y_index
    } else {
        return None;
    };

    let front_gradient_y = depth * 2 - 1;
    let back_gradient_y = front_gradient_y + 2;

    let (rel_x_min, rel_x_max, mut prev_visibility, mut first_iteration) = {
        let double_start_num = min_gradient.y + front_gradient_y * min_gradient.x;
        let double_stop_num = max_gradient.y + back_gradient_y * max_gradient.x;

        let rel_start = double_start_num / (2 * min_gradient.y);

        let stop_denom = 2 * max_gradient.y;
        let rel_stop = if double_stop_num % stop_denom == 0 {
            (double_stop_num - 1) / stop_denom
        } else {
            double_stop_num / stop_denom
        };

        (rel_start, rel_stop, -1.0, true)
    };

    let mut prev_opaque = prev_visibility == 0.0;
    let mut rel_x_index = rel_x_min;

    while rel_x_index <= rel_x_max {
        let coord = if let Some(coord) = octant.make_coord(static_params.centre, rel_x_index, y_index) {
            coord
        } else {
            break;
        };
        let coord_u32 = coord.cast();
        let sh_cell = if let Some(sh_cell) = static_params.spatial_hash.get(coord_u32) {
            sh_cell
        } else {
            break;
        };

        let gradient_x = rel_x_index * 2 - 1;
        let mut direction_bitmap = DirectionBitmap::empty();

        let cur_visibility = (visibility - sh_cell.opacity_total as f32).max(0.0);
        let cur_opaque = cur_visibility == 0.0;

        if cur_opaque {
            // check if we can actually see the facing side
            if max_gradient.x * front_gradient_y > gradient_x * max_gradient.y {
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
            let gradient_y = if cur_visibility < prev_visibility {
                back_gradient_y
            } else {
                front_gradient_y
            };
            let gradient = Vector2::new(gradient_x, gradient_y);

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
        if rel_x_index == rel_x_max {
            if !cur_opaque {
                // see beyond the current section
                next.push(ScanParams {
                    min_gradient,
                    max_gradient,
                    depth: depth + 1,
                    visibility: cur_visibility,
                });
            }
            if visible && max_gradient.x == max_gradient.y {
                return Some(CornerInfo {
                    bitmap: direction_bitmap,
                    coord,
                });
            }
        }

        if visible && octant.should_see(rel_x_index) {
            grid.see(coord_u32, direction_bitmap, static_params.time);
        }

        prev_visibility = cur_visibility;
        prev_opaque = cur_opaque;
        first_iteration = false;
        rel_x_index += 1;
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

fn observe_octant<G: VisionGrid, A: Octant, B: Octant>(grid: &mut G,
                                 env: &mut ShadowcastEnv,
                                 octant_a: A,
                                 octant_b: B,
                                 static_params: &StaticParams)
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


pub fn observe<G: VisionGrid>(grid: &mut G,
                              env: &mut ShadowcastEnv,
                  position: Vector2<f32>, spatial_hash: &SpatialHashTable,
                  distance: u32, time: u64) {
    let coord = (position + Vector2::new(0.5, 0.5)).cast();
    let coord_u32 = coord.cast();

    grid.see(coord_u32, DirectionBitmap::all(), time);

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
