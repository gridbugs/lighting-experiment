use cgmath::Vector2;
use direction::DirectionBitmap;

pub trait VisionGrid {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64);
}
