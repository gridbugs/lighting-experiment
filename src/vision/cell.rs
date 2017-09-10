use direction::Direction;

pub trait VisionCell {
    fn see(&mut self, time: u64);
    fn clear_sides(&mut self);
    fn see_side(&mut self, direction: Direction);
    fn see_all_sides(&mut self);
}
