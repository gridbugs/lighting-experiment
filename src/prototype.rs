use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, insert};
use content::{Sprite, DepthType, DepthInfo};
use append::Append;

pub fn angler<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::player(id));
    changes.append(insert::sprite(id, Sprite::Angler));
    changes.append(insert::depth(id, DepthType::YAxis.into()));
    changes.append(insert::collider(id));
}

pub fn inner_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, Sprite::InnerWall));
    changes.append(insert::depth(id, DepthType::YAxis.into()));
}

pub fn outer_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, Sprite::OuterWall));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::YAxis, -0.75)));
}

pub fn inner_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
}

pub fn outer_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::OuterFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
}

pub fn inner_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerDoor));
    changes.append(insert::depth(id, DepthType::YAxis.into()));
}

pub fn outer_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::OuterDoor));
    changes.append(insert::depth(id, DepthType::YAxis.into()));
}

pub fn window<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Window));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::YAxis, 0.5)));
}
