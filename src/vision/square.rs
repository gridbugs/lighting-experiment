use std::cmp;
use cgmath::Vector2;
use vision::VisionCell;
use grid::GridMut;
use spatial_hash::SpatialHashTable;

pub fn observe<C, G>(grid: &mut G, position: Vector2<f32>, spatial_hash: &SpatialHashTable, distance: u32, time: u64)
    where C: VisionCell,
          G: GridMut<C>,
{
    let position: Vector2<u32> = (position + Vector2::new(0.5, 0.5)).cast();

    for y in cmp::max(position.y - distance, 0)..cmp::min(position.y + distance, spatial_hash.height()) {
        for x in cmp::max(position.x - distance, 0)..cmp::min(position.x + distance, spatial_hash.width()) {
            grid.get_mut(Vector2::new(x, y)).see(time);
        }
    }
}
