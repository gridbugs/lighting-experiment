use cgmath::Vector2;
use direction::Direction;

pub trait VisionGrid {
    type Token: Copy;
    fn get_token(&self, v: Vector2<u32>) -> Self::Token;
    fn see(&mut self, token: Self::Token, time: u64);
    fn clear_sides(&mut self, token: Self::Token);
    fn see_side(&mut self, token: Self::Token, direction: Direction);
    fn see_all_sides(&mut self, token: Self::Token);
}
