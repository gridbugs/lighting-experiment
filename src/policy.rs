use entity_store::{EntityChange, ComponentValue, EntityStore};
use spatial_hash::SpatialHashTable;
use append::Append;
use content::ChangeDesc;

pub fn check<A: Append<ChangeDesc>>(change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable, _reactions: &mut A) -> bool {

    use self::EntityChange::*;
    use self::ComponentValue::*;
    match change {
        &Insert(id, Position(position)) => {
            if let Some(sh_cell) = spatial_hash.get_float(position) {
                if entity_store.collider.contains(&id) &&
                    sh_cell.solid_count > 0 {

                    return false;
                }
            }
        }
        _ => {}
    }

    true
}
