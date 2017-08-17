use enum_primitive::FromPrimitive;
use entity_store::{constants, ComponentType};

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
