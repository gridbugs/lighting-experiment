use content::Sprite;

pub type SpriteAnimation = &'static [SpriteAnimationFrame];

const DOOR_MILLIS: u32 = 16;

use self::Sprite::*;

pub const INNER_DOOR_OPEN: SpriteAnimation = &[
    SpriteAnimationFrame { sprite: InnerDoorOpening1, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening2, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening3, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening4, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening5, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening6, millis: DOOR_MILLIS },
];

pub const INNER_DOOR_CLOSE: SpriteAnimation = &[
    SpriteAnimationFrame { sprite: InnerDoorOpening6, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening5, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening4, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening3, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening2, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: InnerDoorOpening1, millis: DOOR_MILLIS },
];

pub const OUTER_DOOR_OPEN: SpriteAnimation = &[
    SpriteAnimationFrame { sprite: OuterDoorOpening1, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening2, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening3, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening4, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening5, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening6, millis: DOOR_MILLIS },
];

pub const OUTER_DOOR_CLOSE: SpriteAnimation = &[
    SpriteAnimationFrame { sprite: OuterDoorOpening6, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening5, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening4, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening3, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening2, millis: DOOR_MILLIS },
    SpriteAnimationFrame { sprite: OuterDoorOpening1, millis: DOOR_MILLIS },
];

pub struct SpriteAnimationFrame {
    pub sprite: Sprite,
    pub millis: u32,
}

