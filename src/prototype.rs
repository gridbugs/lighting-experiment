use cgmath::Vector2;
use entity_store::{EntityStoreChange, EntityId};
use content::Sprite;
use depth;

pub fn angler(change: &mut EntityStoreChange, id: EntityId, coord: Vector2<u32>) {
    change.position.insert(id, coord.cast());
    change.player.insert(id);
    change.sprite.insert(id, Sprite::Angler);
    change.depth.insert(id, depth::CHARACTER_DEPTH);
}

pub fn outer_wall(change: &mut EntityStoreChange, id: EntityId, coord: Vector2<u32>) {
    change.position.insert(id, coord.cast());
    change.wall.insert(id);
    change.solid.insert(id);
    change.opacity.insert(id, 1.0);
    change.sprite.insert(id, Sprite::OuterWall);
    change.depth.insert(id, depth::WALL_DEPTH);
}

pub fn inner_floor(change: &mut EntityStoreChange, id: EntityId, coord: Vector2<u32>) {
    change.position.insert(id, coord.cast());
    change.sprite.insert(id, Sprite::InnerFloor);
    change.depth.insert(id, depth::FLOOR_DEPTH);
}

pub fn outer_floor(change: &mut EntityStoreChange, id: EntityId, coord: Vector2<u32>) {
    change.position.insert(id, coord.cast());
    change.sprite.insert(id, Sprite::OuterFloor);
    change.depth.insert(id, depth::FLOOR_DEPTH);
}
