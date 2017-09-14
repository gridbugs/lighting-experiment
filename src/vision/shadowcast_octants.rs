use cgmath::Vector2;
use direction::{Direction, DirectionBitmap};

pub trait Octant {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32>;
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32>;
    fn facing_bitmap(&self) -> DirectionBitmap;
    fn across_bitmap(&self) -> DirectionBitmap;
    fn facing_corner_bitmap(&self) -> DirectionBitmap;
    fn should_see(&self, lateral_offset: i32) -> bool;
}

pub struct TopLeft;
pub struct LeftTop;
pub struct TopRight { pub width: u32 }
pub struct RightTop { pub width: u32 }
pub struct BottomLeft { pub height: u32 }
pub struct LeftBottom { pub height: u32 }
pub struct BottomRight { pub width: u32, pub height: u32 }
pub struct RightBottom { pub width: u32, pub height: u32 }

macro_rules! some_if {
    ($value:expr, $condition:expr) => {
        if $condition {
            Some($value)
        } else {
            None
        }
    }
}

macro_rules! see_ahead {
    () => {
        fn should_see(&self, _lateral_offset: i32) -> bool { true }
    }
}

macro_rules! no_see_ahead {
    () => {
        fn should_see(&self, _lateral_offset: i32) -> bool { true }
    }
}

macro_rules! facing {
    ($dirs:expr) => {
        fn facing_bitmap(&self) -> DirectionBitmap {
            $dirs
        }
    }
}

macro_rules! across {
    ($dirs:expr) => {
        fn across_bitmap(&self) -> DirectionBitmap {
            $dirs
        }
    }
}

macro_rules! facing_corner {
    ($dirs:expr) => {
        fn facing_corner_bitmap(&self) -> DirectionBitmap {
            $dirs
        }
    }
}

impl Octant for TopRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(centre.x + lateral_offset, depth_index)
    }
    facing!{Direction::South.bitmap()}
    across!{Direction::West.bitmap()}
    facing_corner!{Direction::SouthWest.bitmap()}
    see_ahead!{}
}

impl Octant for RightTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        some_if!(index, index < self.width as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(depth_index, centre.y - lateral_offset)
    }
    facing!{Direction::West.bitmap()}
    across!{Direction::South.bitmap()}
    facing_corner!{Direction::SouthWest.bitmap()}
    no_see_ahead!{}
}

impl Octant for TopLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(centre.x - lateral_offset, depth_index)
    }
    facing!{Direction::South.bitmap()}
    across!{Direction::East.bitmap()}
    facing_corner!{Direction::SouthEast.bitmap()}
    no_see_ahead!{}
}

impl Octant for LeftTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(depth_index, centre.y - lateral_offset)
    }
    facing!{Direction::East.bitmap()}
    across!{Direction::South.bitmap()}
    facing_corner!{Direction::SouthEast.bitmap()}
    see_ahead!{}
}

impl Octant for BottomLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        some_if!(index, index < self.height as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(centre.x - lateral_offset, depth_index)
    }
    facing!{Direction::North.bitmap()}
    across!{Direction::East.bitmap()}
    facing_corner!{Direction::NorthEast.bitmap()}
    see_ahead!{}
}

impl Octant for LeftBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(depth_index, centre.y + lateral_offset)
    }
    facing!{Direction::East.bitmap()}
    across!{Direction::North.bitmap()}
    facing_corner!{Direction::NorthEast.bitmap()}
    no_see_ahead!{}
}

impl Octant for BottomRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        some_if!(index, index < self.height as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(centre.x + lateral_offset, depth_index)
    }
    facing!{Direction::North.bitmap()}
    across!{Direction::West.bitmap()}
    facing_corner!{Direction::NorthWest.bitmap()}
    no_see_ahead!{}
}

impl Octant for RightBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        some_if!(index, index < self.width as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Vector2<i32> {
        Vector2::new(depth_index, centre.y + lateral_offset)
    }
    facing!{Direction::West.bitmap()}
    across!{Direction::North.bitmap()}
    facing_corner!{Direction::NorthWest.bitmap()}
    see_ahead!{}
}
