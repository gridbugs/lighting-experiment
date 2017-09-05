use content::Sprite;

pub type SpriteAnimation = &'static [SpriteAnimationFrame];

const COMMON_MILLIS: u32 = 32;

use self::Sprite::*;

pub const INNER_DOOR_OPEN: SpriteAnimation = &[
    SpriteAnimationFrame { sprite: InnerDoorOpening1, millis: COMMON_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening2, millis: COMMON_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening3, millis: COMMON_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening4, millis: COMMON_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening5, millis: COMMON_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening6, millis: COMMON_MILLIS },
];

pub struct SpriteAnimationFrame {
    pub sprite: Sprite,
    pub millis: u32,
}

