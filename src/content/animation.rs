use std::time::Duration;
use cgmath::Vector2;

use append::Append;
use entity_store::{EntityId, EntityChange, insert};
use content::{Sprite, SpriteAnimation};

pub enum Animation {
    Slide {
        id: EntityId,
        base: Vector2<f32>,
        path: Vector2<f32>,
        progress: f32,
        duration: Duration,
    },
    Sprites {
        id: EntityId,
        animation: SpriteAnimation,
        final_sprite: Sprite,
        index: usize,
        remaining: Duration,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationStatus {
    Running,
    Finished,
}

fn duration_ratio(a: Duration, b: Duration) -> f32 {
    let a_nanos = (a.subsec_nanos() as u64 + a.as_secs() * 1_000_000_000) as f32;
    let b_nanos = (b.subsec_nanos() as u64 + b.as_secs() * 1_000_000_000) as f32;
    a_nanos / b_nanos
}

impl Animation {
    pub fn populate<A: Append<EntityChange>>(&mut self, time_delta: Duration, changes: &mut A) -> AnimationStatus {
        use self::Animation::*;
        match self {
            &mut Slide { id, base, path, ref mut progress, duration } => {
                let progress_delta = duration_ratio(time_delta, duration);
                *progress += progress_delta;
                if *progress > 1.0 {
                    *progress = 1.0;
                }

                let new_position = base + path * *progress;
                changes.append(insert::position(id, new_position));

                if *progress < 1.0 {
                    AnimationStatus::Running
                } else {
                    AnimationStatus::Finished
                }
            }
            &mut Sprites { id, animation, final_sprite, ref mut index, ref mut remaining } => {
                if time_delta < *remaining {
                    *remaining -= time_delta;
                    return AnimationStatus::Running;
                }

                let mut time_delta_rest = time_delta - *remaining;
                *index += 1;

                loop {
                    if *index == animation.len() {
                        changes.append(insert::sprite(id, final_sprite));
                        return AnimationStatus::Finished;
                    }

                    let frame = &animation[*index];
                    let frame_duration = Duration::from_millis(frame.millis as u64);

                    if time_delta_rest < frame_duration {
                        *remaining = frame_duration - time_delta_rest;
                        changes.append(insert::sprite(id, frame.sprite));
                        break;
                    }

                    time_delta_rest -= frame_duration;
                    *index += 1;
                }

                AnimationStatus::Running
            }
        }
    }
}
