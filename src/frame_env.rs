use entity_store::{EntityStore, EntityStoreChange};
use spatial_hash::SpatialHashTable;
use entity_id_allocator::EntityIdAllocator;

pub struct FrameEnv<'a> {
    pub entity_store: &'a EntityStore,
    pub spatial_hash: &'a SpatialHashTable,
    pub change: &'a mut EntityStoreChange,
    pub allocator: &'a mut EntityIdAllocator,
}
