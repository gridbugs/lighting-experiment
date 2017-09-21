use std::collections::VecDeque;
use cgmath::Vector2;
use static_grid::StaticGrid;
use direction::CardinalDirections;
use spatial_hash::{SpatialHashTable, SpatialHashCell};

struct Cell {
    seq: u64,
    value: u32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            seq: 0,
            value: 0,
        }
    }
}

pub struct DijkstraMap {
    grid: StaticGrid<Cell>,
    coord_queue: VecDeque<Vector2<u32>>,
    seq: u64,
}

impl DijkstraMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: StaticGrid::new_default(width, height),
            coord_queue: VecDeque::new(),
            seq: 0,
        }
    }

    pub fn get_distance(&self, coord: Vector2<i32>) -> Option<u32> {
        self.grid.get_signed(coord).and_then(|cell| {
            if cell.seq == self.seq {
                Some(cell.value)
            } else {
                None
            }
        })
    }

    pub fn compute_distance_to_coord<P>(&mut self, spatial_hash: &SpatialHashTable, coord: Vector2<u32>, threshold: u32,
                                        can_enter: P)
        where P: Fn(&SpatialHashCell) -> bool,
    {
        if !self.grid.contains(coord) {
            return;
        }

        self.seq += 1;

        {
            let cell = self.grid.get_checked_mut(coord);
            cell.seq = self.seq;
            cell.value = 0;
        }
        self.coord_queue.push_back(coord);

        while let Some(coord) = self.coord_queue.pop_front() {
            let value = self.grid.get_checked(coord).value;
            let signed_coord = coord.cast();
            let next_value = value + 1;
            if next_value >= threshold {
                break;
            }

            for direction in CardinalDirections {
                let next_signed_coord = signed_coord + direction.vector();
                if let Some(cell) = self.grid.get_signed_mut(next_signed_coord) {
                    let next_coord = next_signed_coord.cast();
                    let sh_cell = spatial_hash.get(next_coord).expect("Spatial hash of different size to dijkstra map");

                    if cell.seq != self.seq && can_enter(sh_cell) {
                        cell.seq = self.seq;
                        cell.value = next_value;
                        self.coord_queue.push_back(next_coord);
                    }
                }
            }
        }

        self.coord_queue.clear();
    }
}
