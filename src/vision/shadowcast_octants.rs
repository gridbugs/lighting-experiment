use cgmath::Vector2;
use direction::{Direction, DirectionBitmap};
use vision::VisionGrid;
use spatial_hash::SpatialHashTable;

pub trait Octant {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32>;
    fn replace_depth(&self, centre: Vector2<i32>, depth: i32) -> Vector2<i32>;
    fn maybe_handle_first<G: VisionGrid>(&self, grid: &mut G, centre: Vector2<i32>, depth_index: i32,
                                         vision_distance_squared: i32,
                                         visibility: f32, time: u64, spatial_hash: &SpatialHashTable) -> Option<f32>;
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>>;
    fn facing_bitmap(&self) -> DirectionBitmap;
    fn side_bitmap(&self) -> DirectionBitmap;
}

pub struct TopRight { pub width: u32 }
pub struct RightTop { pub width: u32 }
pub struct TopLeft;
pub struct LeftTop;
pub struct BottomLeft { pub height: u32 }
pub struct LeftBottom { pub height: u32 }
pub struct BottomRight { pub width: u32, pub height: u32 }
pub struct RightBottom { pub width: u32, pub height: u32 }

impl Octant for TopRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.y = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, grid: &mut G, centre: Vector2<i32>, depth_index: i32,
                                         vision_distance_squared: i32,
                                         visibility: f32, time: u64, spatial_hash: &SpatialHashTable) -> Option<f32> {
        Some(handle_first(grid, self, centre, depth_index, visibility, vision_distance_squared, time, spatial_hash))
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x + lateral_offset;
        if x < self.width as i32 {
            Some(Vector2::new(x, depth_index))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()
    }
}

impl Octant for RightTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        if index < self.width as i32 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.x = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, _grid: &mut G, _centre: Vector2<i32>, _depth_index: i32,
                                         _vision_distance_squared: i32,
                                         _visibility: f32, _time: u64, _spatial_hash: &SpatialHashTable) -> Option<f32> {
        None
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y - lateral_offset;
        if y > 0 {
            Some(Vector2::new(depth_index, y))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()
    }
}

impl Octant for TopLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.y = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, _grid: &mut G, _centre: Vector2<i32>, _depth_index: i32,
                                         _vision_distance_squared: i32,
                                         _visibility: f32, _time: u64, _spatial_hash: &SpatialHashTable) -> Option<f32> {
        None
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x - lateral_offset;
        if x >= 0 {
            Some(Vector2::new(x, depth_index))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()
    }
}

impl Octant for LeftTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.x = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, grid: &mut G, centre: Vector2<i32>, depth_index: i32,
                                         vision_distance_squared: i32,
                                         visibility: f32, time: u64, spatial_hash: &SpatialHashTable) -> Option<f32> {
        Some(handle_first(grid, self, centre, depth_index, visibility, vision_distance_squared, time, spatial_hash))
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y - lateral_offset;
        if y >= 0 {
            Some(Vector2::new(depth_index, y))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()
    }
}

impl Octant for BottomLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        if index < self.height as i32 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.y = depth;
        centre
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x - lateral_offset;
        if x >= 0 {
            Some(Vector2::new(x, depth_index))
        } else {
            None
        }
    }
    fn maybe_handle_first<G: VisionGrid>(&self, grid: &mut G, centre: Vector2<i32>, depth_index: i32,
                                         vision_distance_squared: i32,
                                         visibility: f32, time: u64, spatial_hash: &SpatialHashTable) -> Option<f32> {
        Some(handle_first(grid, self, centre, depth_index, visibility, vision_distance_squared, time, spatial_hash))
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()
    }
}

impl Octant for LeftBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.x = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, _grid: &mut G, _centre: Vector2<i32>, _depth_index: i32,
                                         _vision_distance_squared: i32,
                                         _visibility: f32, _time: u64, _spatial_hash: &SpatialHashTable) -> Option<f32> {
        None
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y + lateral_offset;
        if y < self.height as i32 {
            Some(Vector2::new(depth_index, y))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()
    }
}

impl Octant for BottomRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        if index < self.height as i32 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.y = depth;
        centre
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x + lateral_offset;
        if x < self.width as i32 {
            Some(Vector2::new(x, depth_index))
        } else {
            None
        }
    }
    fn maybe_handle_first<G: VisionGrid>(&self, _grid: &mut G, _centre: Vector2<i32>, _depth_index: i32,
                                         _vision_distance_squared: i32,
                                         _visibility: f32, _time: u64, _spatial_hash: &SpatialHashTable) -> Option<f32> {
        None
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()
    }
}

impl Octant for RightBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        if index < self.width as i32 {
            Some(index)
        } else {
            None
        }
    }
    fn replace_depth(&self, mut centre: Vector2<i32>, depth: i32) -> Vector2<i32> {
        centre.x = depth;
        centre
    }
    fn maybe_handle_first<G: VisionGrid>(&self, grid: &mut G, centre: Vector2<i32>, depth_index: i32,
                                         vision_distance_squared: i32,
                                         visibility: f32, time: u64, spatial_hash: &SpatialHashTable) -> Option<f32> {
        Some(handle_first(grid, self, centre, depth_index, visibility, vision_distance_squared, time, spatial_hash))
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y + lateral_offset;
        if y < self.height as i32 {
            Some(Vector2::new(depth_index, y))
        } else {
            None
        }
    }
    fn facing_bitmap(&self) -> DirectionBitmap {
        Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()
    }
    fn side_bitmap(&self) -> DirectionBitmap {
        Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()
    }
}



fn handle_first<G: VisionGrid, O: Octant>(grid: &mut G,
                                          octant: &O,
                                          centre: Vector2<i32>,
                                          depth_index: i32,
                                          visibility: f32,
                                          vision_distance_squared: i32,
                                          time: u64,
                                          spatial_hash: &SpatialHashTable) -> f32 {
    let coord = octant.replace_depth(centre, depth_index);
    let coord_u32 = coord.cast();

    let sh_cell = spatial_hash.get(coord_u32).expect("Invalid coord");
    let current_visibility = (visibility - sh_cell.opacity_total as f32).max(0.0);

    let between = coord - centre;
    let distance_squared = between.x * between.x + between.y * between.y;

    if distance_squared < vision_distance_squared {
        let token = grid.get_token(coord_u32);
        grid.see(token, time);

        if current_visibility == 0.0 {
            grid.see_sides(token, octant.facing_bitmap());
        } else {
            grid.see_all_sides(token);
        }
    }

    current_visibility
}
