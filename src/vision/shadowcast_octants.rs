use cgmath::Vector2;
use direction::{Direction, DirectionBitmap};

pub trait Octant {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32>;
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>>;
    fn facing_bitmap(&self) -> DirectionBitmap;
    fn side_bitmap(&self) -> DirectionBitmap;
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

macro_rules! side {
    ($dirs:expr) => {
        fn side_bitmap(&self) -> DirectionBitmap {
            $dirs
        }
    }
}

impl Octant for TopRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x + lateral_offset;
        some_if!(Vector2::new(x, depth_index), x < self.width as i32)
    }
    facing!{Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()}
    side!{Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()}
    see_ahead!{}
}

impl Octant for RightTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        some_if!(index, index < self.width as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y - lateral_offset;
        some_if!(Vector2::new(depth_index, y), y >= 0)
    }
    facing!{Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()}
    side!{Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()}
    no_see_ahead!{}
}

impl Octant for TopLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x - lateral_offset;
        some_if!(Vector2::new(x, depth_index), x >= 0)
    }
    facing!{Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()}
    side!{Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()}
    no_see_ahead!{}
}

impl Octant for LeftTop {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y - lateral_offset;
        some_if!(Vector2::new(depth_index, y), y >= 0)
    }
    facing!{Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()}
    side!{Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()}
    see_ahead!{}
}

impl Octant for BottomLeft {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        some_if!(index, index < self.height as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x - lateral_offset;
        some_if!(Vector2::new(x, depth_index), x >= 0)
    }
    facing!{Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()}
    side!{Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()}
    see_ahead!{}
}

impl Octant for LeftBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x - depth;
        some_if!(index, index >= 0)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y + lateral_offset;
        some_if!(Vector2::new(depth_index, y), y < self.height as i32)
    }
    facing!{Direction::East.bitmap() | Direction::NorthEast.bitmap() | Direction::SouthEast.bitmap()}
    side!{Direction::South.bitmap() | Direction::SouthEast.bitmap() | Direction::SouthWest.bitmap()}
    no_see_ahead!{}
}

impl Octant for BottomRight {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.y + depth;
        some_if!(index, index < self.height as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let x = centre.x + lateral_offset;
        some_if!(Vector2::new(x, depth_index), x < self.width as i32)
    }
    facing!{Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()}
    side!{Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()}
    no_see_ahead!{}
}

impl Octant for RightBottom {
    fn depth_index(&self, centre: Vector2<i32>, depth: i32) -> Option<i32> {
        let index = centre.x + depth;
        some_if!(index, index < self.width as i32)
    }
    fn make_coord(&self, centre: Vector2<i32>, lateral_offset: i32, depth_index: i32) -> Option<Vector2<i32>> {
        let y = centre.y + lateral_offset;
        some_if!(Vector2::new(depth_index, y), y < self.height as i32)
    }
    facing!{Direction::West.bitmap() | Direction::NorthWest.bitmap() | Direction::SouthWest.bitmap()}
    side!{Direction::North.bitmap() | Direction::NorthEast.bitmap() | Direction::NorthWest.bitmap()}
    see_ahead!{}
}