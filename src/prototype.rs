use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, insert};
use content::{Sprite, DepthType};
use append::Append;

pub fn angler<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::player(id));
    changes.append(insert::sprite(id, Sprite::Angler));
    changes.append(insert::depth(id, DepthType::Vertical));
    changes.append(insert::collider(id));
}

pub fn outer_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, Sprite::OuterWall));
    changes.append(insert::depth(id, DepthType::Vertical));
}

pub fn inner_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerFloor));
    changes.append(insert::depth(id, DepthType::Horizontal));
}

pub fn outer_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::OuterFloor));
    changes.append(insert::depth(id, DepthType::Horizontal));
}

pub fn door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Door));
    changes.append(insert::depth(id, DepthType::Vertical));
}
