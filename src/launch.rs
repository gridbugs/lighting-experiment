use frontend::Frontend;
use terrain::demo;
use entity_store::{EntityStore, EntityStoreChange};
use spatial_hash::SpatialHashTable;
use id_allocator::EntityIdAllocator;

pub fn launch(mut frontend: Frontend) {

    let mut allocator = EntityIdAllocator::new();
    let mut change = EntityStoreChange::new();
    let mut entity_store = EntityStore::new();

    let metadata = demo::generate(&mut change, &mut allocator);

    let mut spatial_hash = SpatialHashTable::new(metadata.width, metadata.height);

    spatial_hash.update(&entity_store, &change, 0);
    entity_store.commit_change(&mut change);

    frontend.spin(&entity_store, &spatial_hash);
}
