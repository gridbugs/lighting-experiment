use std::collections::hash_map;
use fnv;

#[path = "macros.gen.rs"]
#[macro_use] mod macros;

#[path = "constants.gen.rs"]
mod constants;

#[cfg(test)]
mod tests;

mod component_type_set;
pub use self::component_type_set::*;

mod entity_vec;
pub use self::entity_vec::*;

pub type EntityMap<T> = EntityVecMap<T>;
pub type EntityMapIter<'a, T> = EntityVecMapIter<'a, T>;
pub type EntitySet = EntityVecSet;

pub type ChangeEntityMap<T> = fnv::FnvHashMap<EntityId, T>;
pub type ChangeEntityMapIter<'a, T> = hash_map::Iter<'a, EntityId, T>;

entity_store_imports!{}

entity_store_decl!{EntityStore}

impl EntityStore {
    pub fn new() -> Self {
        entity_store_cons!(EntityStore)
    }

    pub fn commit(&mut self, entity_change: EntityChange) {
        commit!(self, entity_change)
    }
}

pub type EntityId = u16;

enum_component_type!{ComponentType}
enum_component_value!{ComponentValue}

#[derive(Debug, Clone)]
pub enum Change {
    Insert(ComponentValue),
    Remove(ComponentType),
}

pub struct EntityChange {
    pub id: EntityId,
    pub change: Change,
}

impl EntityChange {
    pub fn new(id: EntityId, change: Change) -> Self {
        Self {
            id,
            change,
        }
    }

    pub fn insert(id: EntityId, value: ComponentValue) -> Self {
        Self::new(id, Change::Insert(value))
    }

    pub fn remove(id: EntityId, typ: ComponentType) -> Self {
        Self::new(id, Change::Remove(typ))
    }
}

insert_shorthands!{insert}
remove_shorthands!{remove}
