use entity_store::{EntityId, EntityStore, insert};
use direction::CardinalDirection;
use append::Append;
use content::{ChangeDesc, DoorState};

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Walk(EntityId, CardinalDirection),
    CloseDoor(EntityId),
}

impl ActionType {
    pub fn populate<A: Append<ChangeDesc>>(self, entity_store: &EntityStore, changes: &mut A) {
        use self::ActionType::*;
        match self {
            Walk(id, dir) => walk(id, dir, entity_store, changes),
            CloseDoor(id) => close_door(id, entity_store, changes),
        }
    }
}

pub fn walk<A: Append<ChangeDesc>>(id: EntityId, dir: CardinalDirection, entity_store: &EntityStore, changes: &mut A) {
    let current_coord = entity_store.coord.get(&id).cloned().expect("Expected coord");
    let new_coord = current_coord + dir.vector();

    changes.append(ChangeDesc::immediate(insert::coord(id, new_coord)));
}

pub fn close_door<A: Append<ChangeDesc>>(id: EntityId, entity_store: &EntityStore, changes: &mut A) {
    let mut info = entity_store.door.get(&id).cloned().expect("Expected door");
    info.state = DoorState::Closed;
    changes.append(ChangeDesc::immediate(insert::door(id, info)));
}
