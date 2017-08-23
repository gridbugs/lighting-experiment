use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, insert};
use content::{Sprite, DepthType};

pub fn angler(changes: &mut Vec<EntityChange>, id: EntityId, position: Vector2<f32>) {
    changes.push(insert::position(id, position));
    changes.push(insert::player(id));
    changes.push(insert::sprite(id, Sprite::Angler));
    changes.push(insert::depth(id, DepthType::Vertical));
}

pub fn outer_wall(changes: &mut Vec<EntityChange>, id: EntityId, position: Vector2<f32>) {
    changes.push(insert::position(id, position));
    changes.push(insert::wall(id));
    changes.push(insert::solid(id));
    changes.push(insert::opacity(id, 1.0));
    changes.push(insert::sprite(id, Sprite::OuterWall));
    changes.push(insert::depth(id, DepthType::Vertical));
}

pub fn inner_floor(changes: &mut Vec<EntityChange>, id: EntityId, position: Vector2<f32>) {
    changes.push(insert::position(id, position));
    changes.push(insert::sprite(id, Sprite::InnerFloor));
    changes.push(insert::depth(id, DepthType::Horizontal));
}

pub fn outer_floor(changes: &mut Vec<EntityChange>, id: EntityId, position: Vector2<f32>) {
    changes.push(insert::position(id, position));
    changes.push(insert::sprite(id, Sprite::OuterFloor));
    changes.push(insert::depth(id, DepthType::Horizontal));
}
