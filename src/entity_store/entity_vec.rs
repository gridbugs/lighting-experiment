use std::mem;
use std::slice;
use std::iter;
use std::vec;
use entity_store::EntityId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityVecMap<T> {
    components: Vec<Option<T>>,
}

impl<T> EntityVecMap<T> {
    pub fn new() -> Self {
        EntityVecMap {
            components: Vec::new(),
        }
    }

    pub fn remove(&mut self, id: &EntityId) -> Option<T> {
        if (*id as usize) >= self.components.len() {
            return None;
        }

        mem::replace(&mut self.components[*id as usize], None)
    }

    pub fn get(&self, id: &EntityId) -> Option<&T> {
        self.components.get(*id as usize).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut T> {
        self.components.get_mut(*id as usize).and_then(Option::as_mut)
    }

    pub fn clear(&mut self) {
        self.components.clear();
    }

    pub fn contains_key(&self, id: &EntityId) -> bool {
        self.get(id).is_some()
    }

    pub fn iter(&self) -> EntityVecMapIter<T> {
        EntityVecMapIter {
            iter: self.components.iter().enumerate(),
        }
    }

    pub fn drain(&mut self) -> EntityVecMapDrain<T> {
        EntityVecMapDrain {
            drain: self.components.drain(..).enumerate(),
        }
    }
}

impl<T: Clone> EntityVecMap<T> {
    pub fn insert(&mut self, id: EntityId, component: T) -> Option<T> {
        if (id as usize) >= self.components.len() {
            self.components.resize(id as usize + 1, None);
        }

        mem::replace(&mut self.components[id as usize], Some(component))
    }
}

impl<T> Default for EntityVecMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntityVecMapIter<'a, T: 'a> {
    iter: iter::Enumerate<slice::Iter<'a, Option<T>>>,
}

impl<'a, T: 'a> Iterator for EntityVecMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, maybe_value)) = self.iter.next() {
            if let Some(value) = maybe_value.as_ref() {
                return Some((id as EntityId, value));
            }
        }

        None
    }
}

pub struct EntityVecMapDrain<'a, T: 'a> {
    drain: iter::Enumerate<vec::Drain<'a, Option<T>>>
}

impl<'a, T: 'a> Iterator for EntityVecMapDrain<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, maybe_value)) = self.drain.next() {
            if let Some(value) = maybe_value {
                return Some((id as EntityId, value));
            }
        }

        None
    }
}
