use cgmath::Vector2;
use static_grid::StaticGrid;
use direction::DirectionBitmap;
use vision::{VisionGrid, VisionGridWithHistory};

pub struct VisionDirectionStore {
    grid: StaticGrid<DirectionBitmap>,
}

impl VisionDirectionStore {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn sticky_vision_grid<G>(&mut self, vision_grid: G) -> StickyVisionGrid<G>
        where G: VisionGrid,
    {
        StickyVisionGrid {
            direction_grid: &mut self.grid,
            vision_grid,
        }
    }
}

pub struct StickyVisionGrid<'a, G: VisionGrid> {
    direction_grid: &'a mut StaticGrid<DirectionBitmap>,
    vision_grid: G,
}

impl<'a, G: VisionGridWithHistory> VisionGrid for StickyVisionGrid<'a, G> {
    fn see(&mut self, v: Vector2<u32>, bitmap: DirectionBitmap, time: u64) {
        let direction_cell = self.direction_grid.get_checked_mut(v);
        *direction_cell |= bitmap;
        self.vision_grid.see_with_history(v, bitmap, *direction_cell, time);
    }
}
