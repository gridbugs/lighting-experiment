use cgmath::Vector2;
use grid::{Grid, GridMut};

pub struct GridSlice<'a, T: 'a> {
    slice: &'a [T],
    width: u32,
}

pub struct GridSliceMut<'a, T: 'a> {
    slice: &'a mut [T],
    width: u32,
}

impl<'a, T> Grid<T> for GridSlice<'a, T> {
    fn get(&self, v: Vector2<u32>) -> &T {
        &self.slice[(v.y * self.width + v.x) as usize]
    }
}

impl<'a, T> GridMut<T> for GridSliceMut<'a, T> {
    fn get_mut(&mut self, v: Vector2<u32>) -> &mut T {
        &mut self.slice[(v.y * self.width + v.x) as usize]
    }
}

impl<'a, T> GridSlice<'a, T> {
    pub fn new(slice: &'a [T], width: u32) -> Self {
        Self { slice, width }
    }
}

impl<'a, T> GridSliceMut<'a, T> {
    pub fn new(slice: &'a mut [T], width: u32) -> Self {
        Self { slice, width }
    }
}
