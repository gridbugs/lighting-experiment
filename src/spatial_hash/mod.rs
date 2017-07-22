// Generated code may contain pattern matches that bind variables
// that are never used
#![allow(unused_variables)]
use std::collections::HashSet;
use entity_store::{EntityId, EntityStore, EntityStoreChange, DataChangeType, FlagChangeType};
use static_grid::{self, StaticGrid, Coord};
use limits::LimitsRect;

#[path = "macros.gen.rs"]
#[macro_use] mod macros;

#[cfg(test)]
mod tests;

spatial_hash_imports!{}

type Position = position_type!();

spatial_hash_cell_decl!{SpatialHashCell}

impl Default for SpatialHashCell {
    fn default() -> Self {
        spatial_hash_cell_cons!{SpatialHashCell}
    }
}

impl SpatialHashCell {
    fn remove(&mut self, entity_id: EntityId, store: &EntityStore) {
        remove!(self, entity_id, store);
    }

    fn insert(&mut self, entity_id: EntityId, store: &EntityStore) {
        insert!(self, entity_id, store);
    }
}

#[derive(Serialize, Deserialize)]
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

    pub fn contains<I: Into<Coord>>(&self, coord: I) -> bool {
        self.grid.contains(coord.into())
    }

    pub fn get<I: Into<Coord>>(&self, coord: I) -> Option<&SpatialHashCell> {
        self.grid.get(coord.into())
    }

    pub fn update(&mut self, store: &EntityStore, change: &EntityStoreChange, time: u64) {
        for (entity_id, change) in position!(change).iter() {
            match change {
                &DataChangeType::Insert(position) => {
                    if let Some(current) = position!(store).get(entity_id) {
                        if let Some(mut cell) = self.grid.get_mut(current.into()) {
                            cell.remove(*entity_id, store);
                            cell.entities.remove(entity_id);
                            cell.last_updated = time;
                        }
                    }
                    if let Some(mut cell) = self.grid.get_mut(position.into()) {
                        cell.insert(*entity_id, store);
                        cell.entities.insert(*entity_id);
                        cell.last_updated = time;
                    }
                }
                &DataChangeType::Remove => {
                    if let Some(current) = position!(store).get(entity_id) {
                        if let Some(mut cell) = self.grid.get_mut(current.into()) {
                            cell.remove(*entity_id, store);
                            cell.entities.remove(entity_id);
                            cell.last_updated = time;
                        }
                    }
                }
            }
        }

        update_component_loops!(self, store, change, time);
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
