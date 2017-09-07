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

                if entity_store.door_opener.contains(&id) {
                    // open doors by bumping into them
                    if let Some(door_id) = sh_cell.door_set.iter().next() {
                        if let Some(mut door_info) = entity_store.door.get(door_id).cloned() {
                            if door_info.state == DoorState::Closed {
                                door_info.state = DoorState::Open;
                                reactions.append(ChangeDesc::immediate(insert::door(*door_id, door_info)));
                                return false;
                            }
                        }
                    }
                }

                if entity_store.collider.contains(&id) {
                    // prevent walking into solid cells
                    if sh_cell.solid_count > 0 {
                        return false;
                    }
                }
            }

            if let Some(current_position) = entity_store.position.get(&id) {
                if position != *current_position {
                    for (door_id, door_info) in entity_store.door.iter() {
                        if let Some(door_position) = entity_store.position.get(door_id) {
                            if *door_position != position {
                                let mut door_info = *door_info;
                                if door_info.state == DoorState::Open {
                                    door_info.state = DoorState::Closed;
                                    reactions.append(ChangeDesc::immediate(insert::door(*door_id, door_info)));
                                }
                            }
                        }
                    }
                }
            }
        }
        &Insert(id, ComponentValue::Door(door_info)) => {
            match door_info.state {
                DoorState::Open => {
                    reactions.append(ChangeDesc::immediate(remove::solid(id)));
                    reactions.append(ChangeDesc::immediate(insert::opacity(id, 0.0)));
                    reactions.append(ChangeDesc::sprites(id, door_info.typ.open_animation(),
                                                         insert::sprite(id, door_info.typ.open_sprite())));
                }
                DoorState::Closed => {
                    reactions.append(ChangeDesc::immediate(insert::solid(id)));
                    reactions.append(ChangeDesc::sprites(id, door_info.typ.close_animation(),
                                     insert::door_closing_finished(id)));
                }
            }
        }
        &Insert(id, ComponentValue::DoorClosingFinished) => {
            reactions.append(ChangeDesc::immediate(insert::opacity(id, 1.0)));
            if let Some(door_info) = entity_store.door.get(&id) {
                reactions.append(ChangeDesc::immediate(insert::sprite(id, door_info.typ.closed_sprite())));
            }
        }
        _ => {}
    }

    true
}
