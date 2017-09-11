use entity_store::{EntityId, EntityChange, ComponentValue, EntityVecMap, EntityStore};
use id_allocator::IdAllocator;

pub type LightId = usize;

pub struct LightManager {
    id_allocator: IdAllocator<LightId>,
    id_table: EntityVecMap<LightId>,
}

impl LightManager {
    pub fn new() -> Self {
        Self {
            id_allocator: IdAllocator::new(),
            id_table: EntityVecMap::new(),
        }
    }

    pub fn light_id(&self, entity_id: EntityId) -> Option<LightId> {
        self.id_table.get(&entity_id).cloned()
    }

    pub fn update(&mut self, entity_change: &EntityChange, entity_store: &EntityStore) {
        use self::ComponentValue::*;
        use self::EntityChange::*;

        match entity_change {
            &Insert(id, Position(_position)) => {
                if let Some(_light_info) = entity_store.light.get(&id) {

                }
            }
            &Insert(id, Light(_light_info)) => {
                let _light_id = if let Some(light_id) = self.id_table.get(&id).cloned() {
                    light_id
                } else {
                    let light_id = self.id_allocator.allocate();
                    self.id_table.insert(id, light_id);
                    light_id
                };
            }
            _ => {}
        }
    }
}
