// Generated code may contain pattern matches that bind variables
// that are never used
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::collections::HashSet;
use fnv;

use entity_store::{EntityId, EntityStore, EntityChange, Change, ComponentValue, ComponentType};
use static_grid::{self, StaticGrid};
use limits::LimitsRect;
use neighbour_count::NeighbourCount;
use direction::Directions;

#[path = "macros.gen.rs"]
#[macro_use] mod macros;

#[cfg(test)]
mod tests;

spatial_hash_imports!{}

spatial_hash_cell_decl!{SpatialHashCell}

impl Default for SpatialHashCell {
    fn default() -> Self {
        spatial_hash_cell_cons!{SpatialHashCell}
    }
}

impl SpatialHashCell {
    fn remove(&mut self, entity_id: EntityId, store: &EntityStore, time: u64) {
        remove!(self, entity_id, store);
        self.entities.remove(&entity_id);
        self.last_updated = time;
    }

    fn insert(&mut self, entity_id: EntityId, store: &EntityStore, time: u64) {
        insert!(self, entity_id, store);
        self.entities.insert(entity_id);
        self.last_updated = time;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpatialHashTable {
    grid: StaticGrid<SpatialHashCell>,
}

impl SpatialHashTable {
    pub fn new(width: u32, height: u32) -> Self {
        SpatialHashTable {
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn width(&self) -> u32 { self.grid.width() }
    pub fn height(&self) -> u32 { self.grid.height() }

    pub fn contains(&self, coord: Vector2<u32>) -> bool {
        self.grid.contains(coord)
    }

    pub fn contains_signed(&self, coord: Vector2<i32>) -> bool {
        self.grid.contains_signed(coord)
    }

    pub fn get(&self, coord: Vector2<u32>) -> Option<&SpatialHashCell> {
        self.grid.get(coord)
    }

    pub fn get_signed(&self, coord: Vector2<i32>) -> Option<&SpatialHashCell> {
        self.grid.get_signed(coord)
    }

    pub fn get_float(&self, position: Vector2<f32>) -> Option<&SpatialHashCell> {
        self.grid.get_signed(position.cast())
    }

    pub fn update(&mut self, store: &EntityStore, entity_change: &EntityChange, time: u64) {
        match &entity_change.change {
            &Change::Insert(ref component) => {
                insert_match!(self, store, entity_change.id, component, time)
            }
            &Change::Remove(typ) => {
                remove_match!(self, store, entity_change.id, typ, time)
            }
        }
    }

    pub fn iter(&self) -> Iter {
        self.grid.iter()
    }

    pub fn coord_iter(&self) -> CoordIter {
        self.grid.coord_iter()
    }
}

pub type Iter<'a> = static_grid::Iter<'a, SpatialHashCell>;
pub type CoordIter = static_grid::CoordIter;

impl LimitsRect for SpatialHashTable {
    fn x_min(&self) -> i32 { self.grid.x_min() }
    fn x_max(&self) -> i32 { self.grid.x_max() }
    fn y_min(&self) -> i32 { self.grid.y_min() }
    fn y_max(&self) -> i32 { self.grid.y_max() }
}
