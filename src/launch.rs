use frontend::Frontend;
use terrain::demo;
use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;
use entity_id_allocator::EntityIdAllocator;

pub fn launch(mut frontend: Frontend) {

    let mut allocator = EntityIdAllocator::new();
    let mut changes = Vec::new();
    let mut entity_store = EntityStore::new();

    let metadata = demo::generate(&mut changes, &mut allocator);

    let mut spatial_hash = SpatialHashTable::new(metadata.width, metadata.height);

    for c in changes.drain(..) {
        spatial_hash.update(&entity_store, &c, 0);
        entity_store.commit(c);
    }

    frontend.spin(&mut entity_store, &mut spatial_hash);
}
