use std::collections::VecDeque;
use cgmath::Vector2;
use static_grid::StaticGrid;
use direction::{CardinalDirection, CardinalDirections};
use spatial_hash::{SpatialHashTable, SpatialHashCell};

pub enum DirectionInfo {
    Direction(CardinalDirection),
    AtDestination,
    NoInformation,
}

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

    pub fn choose_direction(&self, coord: Vector2<u32>) -> DirectionInfo {
        let cell = if let Some(cell) = self.grid.get(coord) {
            cell
        } else {
            return DirectionInfo::NoInformation;
        };

        if cell.seq != self.seq {
            return DirectionInfo::NoInformation;
        }

        let mut best_value = cell.value;
        let mut info = DirectionInfo::AtDestination;

        for direction in CardinalDirections {
            let neighbour_coord = coord.cast() + direction.vector();
            if let Some(neighbour_cell) = self.grid.get_signed(neighbour_coord) {
                if neighbour_cell.seq == self.seq && neighbour_cell.value < best_value {
                    best_value = neighbour_cell.value;
                    info = DirectionInfo::Direction(direction);
                }
            }
        }

        info
    }
}
