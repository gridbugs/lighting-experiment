use std::time::Duration;
use cgmath::Vector2;

use append::Append;
use entity_store::{EntityId, EntityChange, insert};

pub enum Animation {
    Slide {
        id: EntityId,
        base: Vector2<f32>,
        path: Vector2<f32>,
        progress: f32,
        duration: Duration,
    }
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
        }
    }
}
