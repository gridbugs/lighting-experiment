use cgmath::Vector2;
use direction::DirectionBitmap;

pub trait VisionGrid {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64);
}

pub trait VisionGridWithHistory: VisionGrid {
    fn see_with_history(&mut self, v: Vector2<u32>, current: DirectionBitmap, history: DirectionBitmap, time: u64);
}
