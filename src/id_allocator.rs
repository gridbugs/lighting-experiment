use entity_store::EntityId;

pub struct EntityIdAllocator {
    next: EntityId,
    free_list: Vec<EntityId>,
}

impl EntityIdAllocator {
    pub fn new() -> Self {
        EntityIdAllocator {
            next: 0,
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> EntityId {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.next;
            self.next += 1;
            id
        }
    }

    pub fn free(&mut self, id: EntityId) {
        self.free_list.push(id);
    }
}
