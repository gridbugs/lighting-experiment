use std::mem;
use cgmath::Vector2;
use spatial_hash::SpatialHashTable;
use vision::VisionGrid;
use direction::{Direction, DirectionBitmap};

struct ScanParams {
    min_gradient: Vector2<i32>,
    max_gradient: Vector2<i32>,
    depth: i32,
    visibility: f32,
}

trait Octant {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32>;
}

struct TopLeft;

impl Octant for TopLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
}

fn scan<G: VisionGrid, O: Octant>(grid: &mut G,
                       octant: &O,
                       next: &mut Vec<ScanParams>,
                       centre: Vector2<i32>,
                       params: ScanParams,
                       vision_distance_squared: i32,
                       time: u64,
                       spatial_hash: &SpatialHashTable) -> Option<DirectionBitmap>
{
    let ScanParams { mut min_gradient, max_gradient, depth, visibility } = params;

    let y_index = if let Some(y_index) = octant.depth_index(centre, depth) {
        y_index
    } else {
        return None;
    };

    let (x_min, x_max, mut prev_visibility, mut first_iteration) = {
        let double_front_depth = depth * 2 - 1;
        let double_back_depth = depth * 2 + 1;

        let double_centre_x = centre.x * 2 + 1;

        let double_start_num = double_centre_x * min_gradient.y + double_front_depth * min_gradient.x;
        let double_stop_num = double_centre_x * max_gradient.y + double_back_depth * max_gradient.x;

        let start = double_start_num / (2 * min_gradient.y);

        let stop_denom = 2 * max_gradient.y;
        let stop = if double_stop_num % stop_denom == 0 {
            (double_stop_num - 1) / stop_denom
        } else {
            double_stop_num / stop_denom
        };

        if start == centre.x {
            let coord = Vector2::new(start, y_index);
            let coord_u32 = coord.cast();
            if let Some(sh_cell) = spatial_hash.get(coord_u32) {
                let token = grid.get_token(coord_u32);
                grid.see(token, time);
                let current_visibility = (visibility - sh_cell.opacity_total as f32).max(0.0);
                if current_visibility == 0.0 {
                    grid.see_sides(token, Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap());
                } else {
                    grid.see_all_sides(token);
                }
                (start + 1, stop, current_visibility, false)
            } else {
                return None;
            }
        } else {
            (start, stop, -1.0, true)
        }
    };
    let mut prev_opaque = prev_visibility == 0.0;
    let mut x_index = x_min;
    while x_index <= x_max {
        let coord = Vector2::new(x_index, y_index);
        let coord_u32 = coord.cast();
        let sh_cell = if let Some(sh_cell) = spatial_hash.get(coord_u32) {
            sh_cell
        } else {
            break;
        };

        let token = grid.get_token(coord_u32);

        let between = coord - centre;
        let distance_squared = between.x * between.x + between.y * between.y;
        if distance_squared < vision_distance_squared {
            grid.see(token, time);
        }

        let mut direction_bitmap = DirectionBitmap::empty();

        let cur_visibility = (visibility - sh_cell.opacity_total as f32).max(0.0);
        let cur_opaque = cur_visibility == 0.0;

        if cur_opaque {
            direction_bitmap |= Direction::South.bitmap();
        } else {
            direction_bitmap |= DirectionBitmap::all();
        };

        // handle changes in opacity
        if !first_iteration && cur_visibility != prev_visibility {
            let y_offset = if cur_visibility < prev_visibility {
                1
            } else {
                0
            };

            let gradient_x = (x_index - centre.x) * 2 - 1;
            let gradient_y = (depth + y_offset) * 2 - 1;
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
                direction_bitmap |= Direction::West.bitmap();
            }
        }

        if x_index == x_max {
            if !cur_opaque {
                // see beyond the current section
                next.push(ScanParams {
                    min_gradient,
                    max_gradient,
                    depth: depth + 1,
                    visibility: cur_visibility,
                });
            }
            if max_gradient.x == max_gradient.y {
                return Some(direction_bitmap);
            }
        }

        grid.see_sides(token, direction_bitmap);

        prev_visibility = cur_visibility;
        prev_opaque = cur_opaque;
        first_iteration = false;
        x_index += 1;
    }

    None
}

fn observe_octant<G: VisionGrid>(grid: &mut G,
                                 centre: Vector2<i32>,
                                 vision_distance_squared: i32,
                                 time: u64,
                                 spatial_hash: &SpatialHashTable)
{
    let mut next = Vec::new();
    let mut next_swap = Vec::new();

    let mut depth = 1;

    next.push(ScanParams {
        min_gradient: Vector2::new(0, 1),
        max_gradient: Vector2::new(1, 1),
        depth,
        visibility: 1.0,
    });

    loop {
        let mut corner_bitmap = DirectionBitmap::empty();
        while let Some(params) = next.pop() {
            if let Some(direction_bitmap) = scan(grid, &TopLeft, &mut next_swap, centre, params, vision_distance_squared, time, spatial_hash) {
                corner_bitmap |= direction_bitmap;
            }
        }
        let corner_coord = centre + Vector2::new(depth, depth);
        let token = grid.get_token(corner_coord.cast());
        grid.see_sides(token, corner_bitmap);
        depth += 1;

        if next_swap.is_empty() {
            break;
        }
        mem::swap(&mut next, &mut next_swap);
    }
}


pub fn observe<G: VisionGrid>(grid: &mut G,
                  position: Vector2<f32>, spatial_hash: &SpatialHashTable,
                  distance: u32, time: u64) {
    let coord = (position + Vector2::new(0.5, 0.5)).cast();

    let token = grid.get_token(coord);
    grid.see(token, time);
    grid.see_all_sides(token);

    observe_octant(grid, coord.cast(), (distance * distance) as i32, time, spatial_hash);
}
