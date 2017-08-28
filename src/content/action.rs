use entity_store::{EntityId, EntityStore, EntityChange, insert};
use direction::CardinalDirection;
use content::bob;
use append::Append;

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Walk(EntityId, CardinalDirection),
    Bob(EntityId),
}

impl ActionType {
    pub fn populate<A: Append<EntityChange>>(self, entity_store: &EntityStore, changes: &mut A) {
        use self::ActionType::*;
        match self {
            Walk(id, dir) => walk(id, dir, entity_store, changes),
            Bob(id) => bob(id, entity_store, changes),
        }
    }
}

pub fn walk<A: Append<EntityChange>>(id: EntityId, dir: CardinalDirection, entity_store: &EntityStore, changes: &mut A) {
    let current_position = entity_store.position.get(&id).cloned().expect("Expected position");
    let new_position = current_position + dir.vector().cast();
    changes.append(insert::position(id, new_position));
}

pub fn bob<A: Append<EntityChange>>(id: EntityId, entity_store: &EntityStore, changes: &mut A) {
    let current_sprite = entity_store.sprite.get(&id).cloned().expect("Expected sprite");
    let new_sprite = bob::bob_sprite(current_sprite).expect("No bob sprite");
    changes.append(insert::sprite(id, new_sprite));
}
