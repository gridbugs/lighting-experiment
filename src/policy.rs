use entity_store::{EntityChange, ComponentValue, EntityStore, insert, remove};
use spatial_hash::SpatialHashTable;
use append::Append;
use content::{ChangeDesc, DoorState};

pub fn check<A: Append<ChangeDesc>>(change: &EntityChange,
                                    entity_store: &EntityStore,
                                    spatial_hash: &SpatialHashTable,
                                    reactions: &mut A) -> bool {

    use self::EntityChange::*;
    match change {
        &Insert(id, ComponentValue::Position(position)) => {
            if let Some(sh_cell) = spatial_hash.get_float(position) {
                if entity_store.collider.contains(&id) {
                    if let Some(door_id) = sh_cell.door_set.iter().next() {
                        if let Some(mut door_info) = entity_store.door.get(door_id).cloned() {
                            if door_info.state == DoorState::Closed {
                                door_info.state = DoorState::Open;
                                reactions.append(ChangeDesc::immediate(insert::door(*door_id, door_info)));
                                return false;
                            }
                        }
                    }
                    if sh_cell.solid_count > 0 {
                        return false;
                    }
                }
            }
        }
        &Insert(id, ComponentValue::Door(door_info)) => {
            match door_info.state {
                DoorState::Open => {
                    reactions.append(ChangeDesc::immediate(remove::solid(id)));
                    reactions.append(ChangeDesc::sprites(id, door_info.typ.open_animation(), door_info.typ.open_sprite()));
                }
                DoorState::Closed => {

                }
            }
        }
        _ => {}
    }

    true
}
