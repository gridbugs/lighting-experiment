use std::mem;
use std::slice;
use std::iter;
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

    pub fn entry(&mut self, id: &EntityId) -> EntityVecMapEntry<T> {
        if self.contains_key(id) {
            let value = self.get_mut(id).unwrap();
            EntityVecMapEntry::Occupied(value)
        } else {
            EntityVecMapEntry::Vacant {
                map: self,
                id: *id,
            }
        }
    }
}

impl<T: Clone> EntityVecMap<T> {
    pub fn insert(&mut self, id: EntityId, component: T) -> Option<T> {

        if let Some(value) = self.components.get_mut(id as usize) {
            return mem::replace(value, Some(component));
        }

        self.components.resize(id as usize, None);
        self.components.push(Some(component));

        None
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

pub enum EntityVecMapEntry<'a, T: 'a> {
    Occupied(&'a mut T),
    Vacant {
        map: &'a mut EntityVecMap<T>,
        id: EntityId,
    },
}

impl<'a, T: Clone> EntityVecMapEntry<'a, T> {
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            EntityVecMapEntry::Occupied(v) => v,
            EntityVecMapEntry::Vacant { map, id } => {
                map.insert(id, default);
                map.components[id as usize].as_mut().unwrap()
            }
        }
    }
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            EntityVecMapEntry::Occupied(v) => v,
            EntityVecMapEntry::Vacant { map, id } => {
                map.insert(id, default());
                map.components[id as usize].as_mut().unwrap()
            }
        }

    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityVecSet {
    entities: Vec<u64>,
}

impl EntityVecSet {
    pub fn new() -> Self {
        EntityVecSet {
            entities: Vec::new(),
        }
    }

    fn index_mask(id: EntityId) -> (usize, u64) {
        let index = (id / 64) as usize;
        let offset = (id % 64) as u32;
        let mask = (1 as u64) << offset;

        (index, mask)
    }

    pub fn insert(&mut self, id: EntityId) -> bool {
        let (index, mask) = Self::index_mask(id);

        if let Some(bits) = self.entities.get_mut(index) {
            let current = *bits & mask != 0;
            *bits |= mask;
            return current;
        }

        self.entities.resize(index, 0);
        self.entities.push(mask);

        false
    }

    pub fn remove(&mut self, id: &EntityId) -> bool {
        let (index, mask) = Self::index_mask(*id);

        if let Some(bits) = self.entities.get_mut(index) {
            let current = *bits & mask != 0;
            *bits &= !mask;
            current
        } else {
            false
        }
    }

    pub fn contains(&self, id: &EntityId) -> bool {
        let (index, mask) = Self::index_mask(*id);

        if let Some(bits) = self.entities.get(index) {
            bits & mask != 0
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn is_empty(&self) -> bool {
        for bits in self.entities.iter() {
            if *bits != 0 {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> EntityVecSetIter {
        let mut iter = self.entities.iter();
        EntityVecSetIter {
            current: iter.next().map(Clone::clone).unwrap_or(0),
            iter,
            base: 0,
        }
    }
}

impl Default for EntityVecSet {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntityVecSetIter<'a> {
    iter: slice::Iter<'a, u64>,
    current: u64,
    base: EntityId,
}

impl<'a> Iterator for EntityVecSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while self.current == 0 {
            if let Some(current) = self.iter.next() {
                self.current = *current;
                self.base += 64;
            } else {
                return None;
            }
        }

        let trailing = self.current.trailing_zeros();
        self.current &= !(1 << trailing);

        Some(self.base + trailing as u16)
    }
}
