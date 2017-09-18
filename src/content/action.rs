use std::time::Duration;

use entity_store::{EntityId, EntityStore};
use direction::CardinalDirection;
use append::Append;
use content::ChangeDesc;

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Walk(EntityId, CardinalDirection),
}

impl ActionType {
    pub fn populate<A: Append<ChangeDesc>>(self, entity_store: &EntityStore, changes: &mut A) {
        use self::ActionType::*;
        match self {
            Walk(id, dir) => walk(id, dir, entity_store, changes),
        }
    }
}

pub fn walk<A: Append<ChangeDesc>>(id: EntityId, dir: CardinalDirection, entity_store: &EntityStore, changes: &mut A) {
    let current_position = entity_store.position.get(&id).cloned().expect("Expected position");
    let new_position = current_position + dir.vector().cast();

    changes.append(ChangeDesc::slide(id, current_position, new_position, Duration::from_millis(50)));
}
