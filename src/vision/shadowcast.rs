use std::cmp;
use cgmath::Vector2;

use direction::{Direction, CardinalDirection, OrdinalDirection};
use spatial_hash::SpatialHashTable;
use vector_index::VectorIndex;
use vision::VisionGrid;

// Different types of rounding functions
enum RoundType {
    // Round down to the nearest integer
    Floor,

    // Round down to the nearest integer unless the given number
    // is already an integer, in which case subtract 1 from it
    ExclusiveFloor,
}

impl RoundType {
    fn round(&self, x: f64) -> i32 {
        match *self {
            RoundType::Floor => x.floor() as i32,
            RoundType::ExclusiveFloor => (x - 1.0).ceil() as i32,
        }
    }
}

#[derive(PartialEq)]
enum RotationType {
    Clockwise,
    AntiClockwise,
}

const NUM_OCTANTS: usize = 8;

fn cell_centre(coord: Vector2<i32>) -> Vector2<f64> {
    Vector2::new(coord.x as f64 + 0.5, coord.y as f64 + 0.5)
}

fn cell_corner(coord: Vector2<i32>, dir: OrdinalDirection) -> Vector2<f64> {
    match dir {
        OrdinalDirection::NorthEast => Vector2::new((coord.x + 1) as f64, coord.y as f64),
        OrdinalDirection::SouthEast => Vector2::new((coord.x + 1) as f64, (coord.y + 1) as f64),
        OrdinalDirection::SouthWest => Vector2::new(coord.x as f64, (coord.y + 1) as f64),
        OrdinalDirection::NorthWest => Vector2::new(coord.x as f64, coord.y as f64),
    }
}

// Classification of an octant for shadowcast
struct Octant {
    // Direction to proceed with each scan
    depth_dir: Direction,

    // Direction to proceed during a scan
    lateral_dir: Direction,

    // Whether depth_dir is on x or y index
    depth_idx: VectorIndex,

    // Whether lateral_dir is on x or y index
    lateral_idx: VectorIndex,

    // Added to depth part of coord as depth increases
    depth_step: i32,

    // Added to lateral part of coord during scan.
    lateral_step: i32,

    // Copy of lateral_step, casted to a float.
    lateral_step_float: f64,

    // During a scan, if the current cell has more opacity than the
    // previous cell, use the gradient through this corner of the
    // current cell to split the visible area.
    opacity_increase_corner: OrdinalDirection,

    // During a scan, if the current cell has less opacity than the
    // previous cell, use the gradient through this corner of the
    // current cell to split the visible area.
    opacity_decrease_corner: OrdinalDirection,

    // Side of a cell in this octant  facing the eye
    facing_side: Direction,

    // Side of cell facing across eye
    across_side: Direction,

    // Corner of cell closest to eye
    facing_corner: OrdinalDirection,

    // Rounding function to use at the start of a scan to convert a
    // floating point derived from a gradient into part of a coord
    round_start: RoundType,

    // Rounding function to use at the end of a scan to convert a
    // floating point derived from a gradient into part of a coord
    round_end: RoundType,

    // Type of rotation during a scan in this octant
    rotation: RotationType,
}

impl Octant {
    fn new(card_depth_dir: CardinalDirection, card_lateral_dir: CardinalDirection) -> Self {
        let depth_dir = card_depth_dir.direction();
        let lateral_dir = card_lateral_dir.direction();

        let depth_step = VectorIndex::from_card(card_depth_dir).get(depth_dir.vector());
        let lateral_step = VectorIndex::from_card(card_lateral_dir).get(lateral_dir.vector());

        let card_facing_side = card_depth_dir.opposite();
        let card_across_side = card_lateral_dir.opposite();

        let (round_start, round_end) = if lateral_step == 1 {
            (RoundType::Floor, RoundType::ExclusiveFloor)
        } else {
            assert!(lateral_step == -1);
            (RoundType::ExclusiveFloor, RoundType::Floor)
        };

        let rotation = if lateral_dir == depth_dir.left90() {
            RotationType::Clockwise
        } else {
            assert!(depth_dir == lateral_dir.left90());
            RotationType::AntiClockwise
        };

        Octant {
            depth_dir: depth_dir,
            lateral_dir: lateral_dir,

            depth_idx: VectorIndex::from_card(card_depth_dir),
            lateral_idx: VectorIndex::from_card(card_lateral_dir),

            depth_step: depth_step,
            lateral_step: lateral_step,
            lateral_step_float: lateral_step as f64,

            opacity_increase_corner: OrdinalDirection::from_cardinals(card_depth_dir, card_lateral_dir.opposite())
                .expect("Failed to combine directions"),

            opacity_decrease_corner: OrdinalDirection::from_cardinals(
                card_depth_dir.opposite(),
                card_lateral_dir.opposite())
                .expect("Failed to combine directions"),

            facing_side: card_facing_side.direction(),
            across_side: card_across_side.direction(),

            facing_corner: OrdinalDirection::from_cardinals(card_facing_side, card_across_side)
                .expect("Failed to combine directions"),

            round_start: round_start,
            round_end: round_end,

            rotation: rotation,
        }
    }

    fn compute_slope(&self, from: Vector2<f64>, to: Vector2<f64>) -> f64 {
        ((self.lateral_idx.get(to) - self.lateral_idx.get(from)) /
         (self.depth_idx.get(to) - self.depth_idx.get(from)))
            .abs()
    }
}

#[derive(Debug)]
struct Frame {
    depth: u32,
    min_slope: f64,
    max_slope: f64,
    visibility: f64,
}

impl Frame {
    fn new(depth: u32, min_slope: f64, max_slope: f64, visibility: f64) -> Self {
        Frame {
            depth: depth,
            min_slope: min_slope,
            max_slope: max_slope,
            visibility: visibility,
        }
    }
}

struct Limits {
    // limiting coordinates of world
    depth_min: i32,
    depth_max: i32,
    lateral_min: i32,
    lateral_max: i32,

    // eye centre position
    eye_centre: Vector2<f64>,
    eye_lateral_pos: f64,

    // eye index
    eye_depth_idx: i32,
}

impl Limits {
    fn new(eye: Vector2<i32>, world: &SpatialHashTable, octant: &Octant) -> Self {
        let eye_centre = cell_centre(eye);
        let world_limits = (world.width() - 1, world.height() - 1);
        Limits {
            depth_min: 0,
            depth_max:octant.depth_idx.get_tuple(world_limits) as i32,
            lateral_min: 0,
            lateral_max: octant.lateral_idx.get_tuple(world_limits) as i32,
            eye_centre: eye_centre,
            eye_lateral_pos: octant.lateral_idx.get(eye_centre),
            eye_depth_idx: octant.depth_idx.get(eye),
        }
    }
}

struct Scan<'a> {
    depth_idx: i32,
    start_lateral_idx: i32,
    end_lateral_idx: i32,
    limits: &'a Limits,
    frame: &'a Frame,
}

impl<'a> Scan<'a> {
    fn new(limits: &'a Limits,
           frame: &'a Frame,
           octant: &'a Octant,
           distance: u32)
           -> Option<Self> {
        assert!(frame.min_slope >= 0.0);
        assert!(frame.min_slope <= 1.0);
        assert!(frame.max_slope >= 0.0);
        assert!(frame.max_slope <= 1.0);

        // Don't scan past the view distance
        if frame.depth > distance {
            return None;
        }

        // Absolute index in depth direction of current row
        let depth_abs_idx = limits.eye_depth_idx + (frame.depth as i32) * octant.depth_step;

        // Don't scan off the edge of the world
        if depth_abs_idx < limits.depth_min || depth_abs_idx > limits.depth_max {
            return None;
        }

        // Offset of inner side of current row.
        // The 0.5 comes from the fact that the eye is in the centre of its cell.
        let inner_depth_offset = frame.depth as f64 - 0.5;

        // Offset of the outer side of the current row.
        // We add 1 to the inner offset, as row's are 1 unit wide.
        let outer_depth_offset = inner_depth_offset + 1.0;

        // Lateral index to start scanning from.
        // We always scan from from cardinal axis to ordinal axis.
        let rel_scan_start_idx = frame.min_slope * inner_depth_offset;
        let abs_scan_start_idx = octant.round_start
            .round(limits.eye_lateral_pos + rel_scan_start_idx * octant.lateral_step_float);

        // Make sure the scan starts inside the grid.
        // We always scan away from the eye in the lateral direction, so if the scan
        // starts off the grid, the entire scan will be off the grid, so can be skipped.
        if abs_scan_start_idx < limits.lateral_min || abs_scan_start_idx > limits.lateral_max {
            return None;
        }

        // Lateral index at which to stop scanning.
        let rel_scan_end_idx = frame.max_slope * outer_depth_offset;
        let abs_scan_end_idx = octant.round_end
            .round(limits.eye_lateral_pos + rel_scan_end_idx * octant.lateral_step_float);

        // Constrain the end of the scan within the limits of the grid
        let abs_scan_end_idx = cmp::min(cmp::max(abs_scan_end_idx, limits.lateral_min),
                                        limits.lateral_max);

        Some(Scan {
            depth_idx: depth_abs_idx,
            start_lateral_idx: abs_scan_start_idx,
            end_lateral_idx: abs_scan_end_idx,
            limits: limits,
            frame: frame,
        })
    }
}

struct OctantArgs<'a> {
    octant: &'a Octant,
    world: &'a SpatialHashTable,
    eye: Vector2<i32>,
    distance: u32,
    distance_squared: i32,
    initial_min_slope: f64,
    initial_max_slope: f64,
    time: u64,
}

impl<'a> OctantArgs<'a> {
    fn new(octant: &'a Octant,
           world: &'a SpatialHashTable,
           eye: Vector2<i32>,
           distance: u32,
           initial_min_slope: f64,
           initial_max_slope: f64,
           time: u64)
           -> Self {
        OctantArgs {
            octant,
            world,
            eye,
            distance,
            distance_squared: (distance * distance) as i32,
            initial_min_slope,
            initial_max_slope,
            time,
        }
    }
}

pub struct ShadowcastEnv {
    octants: [Octant; NUM_OCTANTS],
    stack: Vec<Frame>,
}

impl ShadowcastEnv {
    pub fn new() -> Self {
        ShadowcastEnv {
            // The order octants appear is the order one would visit
            // each octant if they started at -PI radians and moved
            // in the positive (anticlockwise) direction.
            octants: [Octant::new(CardinalDirection::West, CardinalDirection::South),
                      Octant::new(CardinalDirection::South, CardinalDirection::West),
                      Octant::new(CardinalDirection::South, CardinalDirection::East),
                      Octant::new(CardinalDirection::East, CardinalDirection::South),
                      Octant::new(CardinalDirection::East, CardinalDirection::North),
                      Octant::new(CardinalDirection::North, CardinalDirection::East),
                      Octant::new(CardinalDirection::North, CardinalDirection::West),
                      Octant::new(CardinalDirection::West, CardinalDirection::North),],
            stack: Vec::new(),
        }
    }
}

fn scan<G>(grid: &mut G, stack: &mut Vec<Frame>, args: &OctantArgs, scan: &Scan)
    where G: VisionGrid,
{
    let mut coord = args.octant.depth_idx.create_coord(scan.depth_idx);

    let mut first_iteration = true;
    let mut previous_opaque = false;
    let mut previous_visibility = -1.0;
    let mut idx = scan.start_lateral_idx;
    let mut min_slope = scan.frame.min_slope;

    let final_idx = scan.end_lateral_idx + args.octant.lateral_step;

    while idx != final_idx {

        let last_iteration = idx == scan.end_lateral_idx;

        // update the coord to the current grid position
        args.octant.lateral_idx.set(&mut coord, idx);

        // look up spatial hash cell
        let cell = match args.world.get(coord.cast()) {
            Some(c) => c,
            None => {
                idx += args.octant.lateral_step;
                continue;
            }
        };

        let token = grid.get_token(coord.cast());

        // report the cell as visible
        let between = coord - args.eye;
        let distance_squared = between.x * between.x + between.y * between.y;
        if distance_squared < args.distance_squared {
            grid.see(token, args.time);
            grid.clear_sides(token);
        }

        // compute current visibility
        let current_visibility = (scan.frame.visibility - cell.opacity_total).max(0.0);
        let current_opaque = current_visibility == 0.0;

        if current_opaque {
            if !last_iteration {
                grid.see_side(token, args.octant.facing_side);
            }
        } else {
            grid.see_all_sides(token);
        }

        // process changes in visibility
        if !first_iteration {
            // determine corner of current cell we'll be looking through
            let corner = if current_visibility > previous_visibility {
                Some(args.octant.opacity_decrease_corner)
            } else if current_visibility < previous_visibility {
                Some(args.octant.opacity_increase_corner)
            } else {
                // no change in visibility - nothing happens
                None
            };

            if let Some(corner) = corner {
                let corner_coord = cell_corner(coord, corner);
                let slope = args.octant.compute_slope(scan.limits.eye_centre, corner_coord);
                assert!(slope >= 0.0);
                assert!(slope <= 1.0);

                if !previous_opaque {
                    // unless this marks the end of an opaque region, push
                    // the just-completed region onto the stack so it can
                    // be expanded in a future scan
                    stack.push(Frame::new(scan.frame.depth + 1,
                                         min_slope,
                                         slope,
                                         previous_visibility));
                }

                min_slope = slope;

                if current_opaque {
                    grid.see_side(token, args.octant.across_side);
                }
            }
        }

        if last_iteration {
            if current_opaque {
                let corner_coord = cell_corner(coord, args.octant.facing_corner);
                let slope = args.octant.compute_slope(scan.limits.eye_centre, corner_coord);
                if scan.frame.max_slope > slope {
                    grid.see_side(token, args.octant.facing_side);
                } else if scan.frame.max_slope == slope {
                    grid.see_side(token, args.octant.facing_corner.direction());
                }
            } else {
                // push the final region of the scan to the stack
                stack.push(Frame::new(scan.frame.depth + 1,
                                     min_slope,
                                     scan.frame.max_slope,
                                     current_visibility));
            }
        }

        previous_opaque = current_opaque;
        previous_visibility = current_visibility;
        first_iteration = false;

        idx += args.octant.lateral_step;
    }
}

fn detect_visible_area_octant<G>(grid: &mut G, stack: &mut Vec<Frame>,
                                 args: &OctantArgs)
    where G: VisionGrid,
{
    let limits = Limits::new(args.eye, args.world, args.octant);

    // Initial stack frame
    stack.push(Frame::new(1, args.initial_min_slope, args.initial_max_slope, 1.0));

    while let Some(frame) = stack.pop() {
        if let Some(scan_desc) = Scan::new(&limits, &frame, args.octant, args.distance) {
            // Scan::new can yield None if the scan would be entirely off the grid
            // outside the view distance.
            scan(grid, stack, args, &scan_desc);
        }
    }
}

pub fn observe<G>(grid: &mut G, env: &mut ShadowcastEnv,
                  position: Vector2<f32>, spatial_hash: &SpatialHashTable,
                  distance: u32, time: u64)
    where G: VisionGrid,
{
    let position: Vector2<u32> = (position + Vector2::new(0.5, 0.5)).cast();

    let token = grid.get_token(position);
    grid.see(token, time);

    for octant in env.octants.iter() {
        let args = OctantArgs::new(octant, spatial_hash, position.cast(), distance, 0.0, 1.0, time);
        detect_visible_area_octant(grid, &mut env.stack, &args);
    }
}
