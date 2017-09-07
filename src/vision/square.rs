use std::cmp;
use cgmath::Vector2;
use vision::VisionCell;
use grid::GridMut;
use spatial_hash::SpatialHashTable;

const RANGE: i32 = 8;

pub fn observe<C, G>(grid: &mut G, position: Vector2<f32>, spatial_hash: &SpatialHashTable, time: u64)
    where C: VisionCell,
          G: GridMut<C>,
{
    let position: Vector2<i32> = (position + Vector2::new(0.5, 0.5)).cast();

    for y in cmp::max(position.y - RANGE, 0)..cmp::min(position.y + RANGE, spatial_hash.height() as i32) {
        for x in cmp::max(position.x - RANGE, 0)..cmp::min(position.x + RANGE, spatial_hash.width() as i32) {
            grid.get_mut(Vector2::new(x as u32, y as u32)).see(time);
        }
    }
}
