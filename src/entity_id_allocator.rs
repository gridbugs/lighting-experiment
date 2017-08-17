use entity_store::EntityId;

pub struct EntityIdAllocator {
    next: EntityId,
}

impl EntityIdAllocator {
    pub fn new() -> Self {
        EntityIdAllocator {
            next: 0,
        }
    }
    pub fn allocate(&mut self) -> EntityId {
        let entity_id = self.next;
        self.next += 1;
        entity_id
    }
}
