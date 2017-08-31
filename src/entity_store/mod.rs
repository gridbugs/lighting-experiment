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
