use content::{TileSprite, SpriteAnimation, sprite_animation, DoorState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorType {
    Inner,
    Outer,
}

use self::DoorType::*;

impl DoorType {
    pub fn open_sprite(self) -> TileSprite {
        match self {
            Inner => TileSprite::InnerDoorOpen,
            Outer => TileSprite::OuterDoorOpen,
        }
    }

    pub fn closed_sprite(self) -> TileSprite {
        match self {
            Inner => TileSprite::InnerDoor,
            Outer => TileSprite::OuterDoor,
        }
    }

    pub fn state_sprite(self, state: DoorState) -> TileSprite {
        match state {
            DoorState::Open => self.open_sprite(),
            DoorState::Closed => self.closed_sprite(),
        }
    }

    pub fn open_animation(self) -> SpriteAnimation {
        match self {
            Inner => sprite_animation::INNER_DOOR_OPEN,
            Outer => sprite_animation::OUTER_DOOR_OPEN,
        }
    }

    pub fn close_animation(self) -> SpriteAnimation {
        match self {
            Inner => sprite_animation::INNER_DOOR_CLOSE,
            Outer => sprite_animation::OUTER_DOOR_CLOSE,
        }
    }
}
