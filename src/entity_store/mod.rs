use std::collections::hash_map;
use fnv;

#[path = "macros.gen.rs"]
#[macro_use] mod macros;

#[path = "constants.gen.rs"]
#[macro_use] mod constants;

#[macro_use] pub mod post_change;
#[macro_use] pub mod migration;

#[cfg(test)]
mod tests;

mod component_type_set;
pub use self::component_type_set::*;

mod entity_vec;
pub use self::entity_vec::*;

pub type EntityMap<T> = EntityVecMap<T>;
pub type EntityMapIter<'a, T> = EntityVecMapIter<'a, T>;
pub type EntitySet = fnv::FnvHashSet<EntityId>;

pub type ChangeEntityMap<T> = fnv::FnvHashMap<EntityId, T>;
pub type ChangeEntityMapIter<'a, T> = hash_map::Iter<'a, EntityId, T>;

entity_store_imports!{}

entity_store_decl!{EntityStore}

impl EntityStore {
    pub fn new() -> Self {
        entity_store_cons!(EntityStore)
    }

    pub fn commit_change(&mut self, change: &mut EntityStoreChange) {
        commit_change!(self, change)
    }

    pub fn commit_change_into_change(&mut self, change: &mut EntityStoreChange, dest: &mut EntityStoreChange) {
        commit_change_into!(self, change, dest)
    }

    pub fn commit_change_into_store(&mut self, change: &mut EntityStoreChange, dest: &mut EntityStore) {
        commit_change_into!(self, change, dest)
    }
}

pub type EntityId = u64;

#[derive(Debug, Clone, Copy)]
pub enum DataChangeType<T> {
    Insert(T),
    Remove,
}

#[derive(Debug, Clone, Copy)]
pub enum FlagChangeType {
    Insert,
    Remove,
}

#[derive(Debug, Clone)]
pub struct DataComponentChange<T>(ChangeEntityMap<DataChangeType<T>>);
#[derive(Debug, Clone)]
pub struct FlagComponentChange(ChangeEntityMap<FlagChangeType>);

pub type DataComponentChangeIter<'a, T> = ChangeEntityMapIter<'a, DataChangeType<T>>;
pub type FlagComponentChangeIter<'a> = ChangeEntityMapIter<'a, FlagChangeType>;

impl<T> DataComponentChange<T> {
    pub fn get(&self, id: &EntityId) -> Option<&DataChangeType<T>> {
        self.0.get(&id)
    }
    pub fn iter(&self) -> DataComponentChangeIter<T> {
        self.0.iter()
    }
    pub fn insert(&mut self, id: EntityId, value: T) {
        self.0.insert(id, DataChangeType::Insert(value));
    }
    pub fn remove(&mut self, id: EntityId) {
        self.0.insert(id, DataChangeType::Remove);
    }
    pub fn cancel(&mut self, id: EntityId) -> Option<DataChangeType<T>> {
        self.0.remove(&id)
    }
}
impl FlagComponentChange {
    pub fn iter(&self) -> FlagComponentChangeIter {
        self.0.iter()
    }
    pub fn insert(&mut self, id: EntityId) {
        self.0.insert(id, FlagChangeType::Insert);
    }
    pub fn remove(&mut self, id: EntityId) {
        self.0.insert(id, FlagChangeType::Remove);
    }
    pub fn cancel(&mut self, id: EntityId) -> Option<FlagChangeType> {
        self.0.remove(&id)
    }
}

entity_store_change_decl!{EntityStoreChange}

impl EntityStoreChange {
    pub fn new() -> Self {
        entity_store_change_cons!(EntityStoreChange)
    }
    pub fn remove_entity(&mut self, entity: EntityId, store: &EntityStore) {
        remove_entity!(self, entity, store);
    }
    pub fn clear(&mut self) {
        entity_store_change_clear!(self);
    }
}

enum_component_type!{ComponentType}
enum_component_value!{ComponentValue}
