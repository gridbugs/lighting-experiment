use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, insert};
use content::{Sprite, DepthType, DepthInfo, DoorState, DoorInfo,
              DoorType, SpriteEffectInfo, LightInfo};
use append::Append;

pub fn angler<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Angler));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.39)));
    changes.append(insert::collider(id));
    changes.append(insert::player(id));
    changes.append(insert::door_opener(id));
    changes.append(insert::light(id, LightInfo::new(0.2, 20, 1.0, 1.0, 1.0, 1.0)));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
}

pub fn crab<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Crab));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.4)));
    changes.append(insert::collider(id));
    changes.append(insert::door_opener(id));
    changes.append(insert::npc(id));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
}

pub fn snail<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Snail));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.4)));
    changes.append(insert::collider(id));
    changes.append(insert::door_opener(id));
    changes.append(insert::npc(id));
    changes.append(insert::bump_attack(id));
    changes.append(insert::attackable(id));
}

pub fn inner_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, Sprite::InnerWall));
    changes.append(insert::depth(id, DepthType::Fixed.into()));
}

pub fn outer_wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::wall(id));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
    changes.append(insert::sprite(id, Sprite::OuterWall));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, -0.75)));
}

pub fn inner_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
}

pub fn inner_water<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerWater));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Gradient, 0.01)));
    changes.append(insert::sprite_effect(id, SpriteEffectInfo::water(6, 0.3, 0.7)));
}

pub fn outer_floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::OuterFloor));
    changes.append(insert::depth(id, DepthType::Bottom.into()));
    changes.append(insert::sprite_effect(id, SpriteEffectInfo::water(3, 0.2, 0.8)));
}

pub fn inner_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::InnerDoor));
    changes.append(insert::depth(id, DepthType::Gradient.into()));
    changes.append(insert::door(id, DoorInfo::new(DoorType::Inner, DoorState::Closed)));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
}

pub fn outer_door<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::OuterDoor));
    changes.append(insert::depth(id, DepthType::Gradient.into()));
    changes.append(insert::door(id, DoorInfo::new(DoorType::Outer, DoorState::Closed)));
    changes.append(insert::solid(id));
    changes.append(insert::opacity(id, 1.0));
}

pub fn window<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Window));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, 0.5)));
    changes.append(insert::opacity(id, -1.0));
}

pub fn light<A: Append<EntityChange>>(changes: &mut A, id: EntityId, position: Vector2<f32>, colour: [f32; 3]) {
    changes.append(insert::position(id, position));
    changes.append(insert::sprite(id, Sprite::Light));
    changes.append(insert::depth(id, DepthInfo::new(DepthType::Fixed, 0.0)));
    changes.append(insert::light(id, LightInfo::new(1.0, 20, 2.0, colour[0], colour[1], colour[2])));
}
