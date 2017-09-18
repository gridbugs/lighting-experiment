use std::time::Duration;
use cgmath::Vector2;

use entity_store::{EntityId, EntityChange, insert};
use content::{Animation, SpriteAnimation};

pub enum ChangeDesc {
    // The change is applied immediately.
    Immediate(EntityChange),

    // The change is never applied directly, but the animation should result in the change
    // eventually. The change is used to check if the animation should be run.
    AnimatedChange(EntityChange, Animation),

    // The animation will be run regardless of its outcome.
    Animation(Animation),
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
        ChangeDesc::AnimatedChange(eventual_change, animation)
    }
    pub fn bump_slide(id: EntityId, from: Vector2<f32>, target: Vector2<f32>, duration: Duration, turnaround_progress: f32) -> Self {
        let animation = Animation::BumpSlide {
            id,
            base: from,
            path: target - from,
            progress: 0.0,
            duration,
            turnaround_progress,
        };
        ChangeDesc::Animation(animation)
    }
    pub fn sprites(id: EntityId, animation: SpriteAnimation, then: EntityChange) -> Self {
        let animation = Animation::Sprites {
            id,
            then,
            animation,
            index: 0,
            remaining: Duration::from_millis(animation[0].millis as u64),
        };
        ChangeDesc::Animation(animation)
    }
}
