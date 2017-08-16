use std::collections::hash_map;
use fnv;
use enum_primitive::FromPrimitive;

#[path = "macros.gen.rs"]
#[macro_use] mod macros;

#[path = "constants.gen.rs"]
#[macro_use] mod constants;

#[macro_use] pub mod post_change;
#[macro_use] pub mod migration;

#[cfg(test)]
mod tests;

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
pub struct DataComponentChange<T>(fnv::FnvHashMap<EntityId, DataChangeType<T>>);
#[derive(Debug, Clone)]
pub struct FlagComponentChange(fnv::FnvHashMap<EntityId, FlagChangeType>);

pub type DataComponentChangeIter<'a, T> = hash_map::Iter<'a, EntityId, DataChangeType<T>>;
pub type FlagComponentChangeIter<'a> = hash_map::Iter<'a, EntityId, FlagChangeType>;

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

#[derive(Debug, Clone, Copy)]
pub struct ComponentTypeSet {
    bitmaps: [u64; constants::NUM_COMPONENT_TYPE_WORDS],
}

impl ComponentTypeSet {
    pub fn new() -> Self {
        ComponentTypeSet {
            bitmaps: [0; constants::NUM_COMPONENT_TYPE_WORDS],
        }
    }

    pub fn is_empty(&self) -> bool {
        for b in self.bitmaps.iter() {
            if *b != 0 {
                return false;
            }
        }
        true
    }

    pub fn insert(&mut self, component_type: ComponentType) {
        self.bitmaps[(component_type as usize) / 64]
            |= 1 << ((component_type as usize) % 64);
    }

    pub fn remove(&mut self, component_type: ComponentType) {
        self.bitmaps[(component_type as usize) / 64]
            &= !(1 << ((component_type as usize) % 64));
    }

    pub fn contains(&self, component_type: ComponentType) -> bool {
        self.bitmaps[(component_type as usize) / 64] &
            (1 << ((component_type as usize % 64))) != 0
    }

    pub fn iter(&self) -> ComponentTypeSetIter {
        ComponentTypeSetIter {
            bitmaps: self.bitmaps,
            index: 0,
        }
    }
}

pub struct ComponentTypeSetIter {
    bitmaps: [u64; constants::NUM_COMPONENT_TYPE_WORDS],
    index: usize,
}

impl Iterator for ComponentTypeSetIter {
    type Item = ComponentType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < constants::NUM_COMPONENT_TYPE_WORDS &&
            self.bitmaps[self.index] == 0
        {
            self.index += 1;
        }
        if self.index == constants::NUM_COMPONENT_TYPE_WORDS {
            return None;
        }

        let trailing = self.bitmaps[self.index].trailing_zeros();
        self.bitmaps[self.index] &= !(1 << trailing);
        let component_type_num = trailing + (self.index as u32) * 64;
        let component_type = ComponentType::from_u32(component_type_num)
            .expect("Failed to form ComponentType from ComponentTypeSetIter");

        Some(component_type)
    }
}
