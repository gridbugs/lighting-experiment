use cgmath::Vector2;

pub trait Grid<T> {
    fn get(&self, v: Vector2<u32>) -> &T;
}

pub trait GridMut<T> {
    fn get_mut(&mut self, v: Vector2<u32>) -> &mut T;
}
