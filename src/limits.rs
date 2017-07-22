use std::cmp;
use cgmath::Vector2;

pub trait LimitsRect {
    fn x_min(&self) -> i32;
    fn x_max(&self) -> i32;
    fn y_min(&self) -> i32;
    fn y_max(&self) -> i32;

    fn checked(&self, v: Vector2<i32>) -> Option<Vector2<i32>> {
        if v.x >= self.x_min() && v.x <= self.x_max() &&
            v.y >= self.y_min() && v.y <= self.y_max()
        {
            Some(v)
        } else {
            None
        }
    }

    fn saturate(&self, mut v: Vector2<i32>) -> Vector2<i32> {
        v.x = cmp::max(self.x_min(), cmp::min(self.x_max(), v.x));
        v.y = cmp::max(self.y_min(), cmp::min(self.y_max(), v.y));
        v
    }
}
