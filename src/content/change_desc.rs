use std::time::Duration;
use cgmath::Vector2;

use entity_store::{EntityId, EntityChange, insert};
use content::Animation;

pub enum ChangeDesc {
    Immediate(EntityChange),
    Animation(EntityChange, Animation),
}

impl ChangeDesc {
    pub fn immediate(change: EntityChange) -> Self {
        ChangeDesc::Immediate(change)
    }
    pub fn slide(id: EntityId, from: Vector2<f32>, to: Vector2<f32>, duration: Duration) -> Self {
        let eventual_change = insert::position(id, to);
        let animation = Animation::Slide {
            id,
            base: from,
            path: to - from,
            progress: 0.0,
            duration,
        };
        ChangeDesc::Animation(eventual_change, animation)
    }
}
