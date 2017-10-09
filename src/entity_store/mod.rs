#[path = "macros.gen.rs"]
#[macro_use] mod macros;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[path = "constants.gen.rs"]
mod constants;

#[cfg(test)]
mod tests;

mod component_type_set;
pub use self::component_type_set::*;

mod entity_vec;
pub use self::entity_vec::*;

pub type EntityHashMap<T> = HashMap<EntityId, T>;
pub type EntityHashSet = HashSet<EntityId>;

pub type EntityBTreeMap<T> = BTreeMap<EntityId, T>;
pub type EntityBTreeSet = BTreeSet<EntityId>;

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
enum_component_value_types!{ComponentValue, ComponentType}

#[derive(Debug, Clone)]
pub enum EntityChange {
    Insert(EntityId, ComponentValue),
    Remove(EntityId, ComponentType),
}

impl EntityChange {
    pub fn id(&self) -> EntityId {
        match self {
            &EntityChange::Insert(id, ..) => id,
            &EntityChange::Remove(id, ..) => id,
        }
    }
}

insert_shorthands!{insert}
remove_shorthands!{remove}
