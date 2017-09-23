use std::cmp::Ordering;
use std::collections::BinaryHeap;
use cgmath::Vector2;
use static_grid::StaticGrid;
use spatial_hash::{SpatialHashTable, SpatialHashCell};
use direction::{CardinalDirection, CardinalDirections};

struct Node {
    cost: u32,
    score: u32,
    coord: Vector2<u32>,
}

struct Cell {
    seq: u64,
    enter_direction: Option<CardinalDirection>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            seq: 0,
            enter_direction: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

pub struct SearchEnv {
    grid: StaticGrid<Cell>,
    queue: BinaryHeap<Node>,
    seq: u64,
}

impl SearchEnv {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: StaticGrid::new_default(width, height),
            queue: BinaryHeap::new(),
            seq: 0,
        }
    }

    pub fn search_start<P>(&mut self,
                           spatial_hash: &SpatialHashTable,
                           start: Vector2<i32>,
                           end: Vector2<i32>,
                           can_enter: P) -> Option<CardinalDirection>
        where P: Fn(&SpatialHashCell, Vector2<u32>) -> bool,
    {
        if start == end {
            println!("start and end are the same");
            return None;
        }
        let (start, end) = if let Some((start, end)) = self.search_bounds(start, end) {
            (start, end)
        } else {
            return None;
        };

        println!("searching from {:?} to {:?}", start, end);
        self.seq += 1;
        self.queue.clear();
        self.queue.push(Node {
            cost: 0,
            score: 0,
            coord: start,
        });
        {
            let cell = self.grid.get_checked_mut(start);
            cell.seq = self.seq;
            cell.enter_direction = None;
        }

        let mut found = false;

        'outer: while let Some(node) = self.queue.pop() {
            println!("expanded {:?}", node.coord);
            let signed_coord = node.coord.cast();
            let cost = node.cost + 1;

            for direction in CardinalDirections {
                let next_signed_coord = signed_coord + direction.vector();
                if let Some(cell) = self.grid.get_signed_mut(next_signed_coord) {
                    if cell.seq != self.seq {
                        let next_coord = next_signed_coord.cast();
                        let sh_cell = spatial_hash.get(next_coord).expect("Spatial hash of different size to dijkstra map");
                        if can_enter(sh_cell, next_coord) {
                            println!("visited {:?}", next_coord);
                            cell.seq = self.seq;
                            cell.enter_direction = Some(direction);
                            if next_coord == end {
                                // found path to dest
                                found = true;
                                break 'outer;
                            }
                            self.queue.push(Node {
                                cost,
                                score: cost,
                                coord: next_coord,
                            });
                        }
                    }
                }
            }
        }

        if found {
            println!("found {:?}", end);
            let mut coord = end;
            let mut last_direction = CardinalDirection::North;
            loop {
                let cell = self.grid.get_checked(coord);
                if let Some(direction) = cell.enter_direction {
                    let enter_direction = direction.opposite();
                    coord = (coord.cast() + enter_direction.vector()).cast();
                    last_direction = direction;
                    println!("  -> {:?}", coord);
                } else {
                    println!("{:?}", last_direction);
                    return Some(last_direction);
                }
            }
        }

        None
    }

    fn search_bounds(&self, start_coord: Vector2<i32>, end_coord: Vector2<i32>) -> Option<(Vector2<u32>, Vector2<u32>)> {
        let start_coord = if let Some(start_coord) = self.grid.convert_signed(start_coord) {
            start_coord
        } else {
            println!("failed to convert start");
            return None;
        };
        let end_coord = if let Some(end_coord) = self.grid.convert_signed(end_coord) {
            end_coord
        } else {
            println!("failed to convert end");
            return None;
        };
        Some((start_coord, end_coord))
    }
}
