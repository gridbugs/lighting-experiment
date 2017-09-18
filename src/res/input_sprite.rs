use std::collections::BTreeMap;
use cgmath::{Vector2, ElementWise};
use direction::Direction;
use content::Sprite;

pub const WIDTH_PX: u32 = 16;
pub const HEIGHT_PX: u32 = 16;
pub const DIMENSIONS: Vector2<u32> = Vector2 { x: WIDTH_PX, y: HEIGHT_PX };

pub fn input_sprites() -> Vec<InputSprite> {

    use self::Sprite::*;

    vec![
        character(Angler, [0, 0], Some([0, 8]), None),
        character(Crab, [1, 0], Some([0, 8]), None),
        floor(InnerFloor, [0, 0], None, None),
        floor(OuterFloor, [0, 1], None, None),
        floor(InnerWater, [1, 1], None, None),
        wall(InnerWall, [0, 0], None, None),
        wall(OuterWall, [0, 1], None, None),

        door(InnerDoor, [0, 0]),
        door(InnerDoorOpening1, [1, 0]),
        door(InnerDoorOpening2, [2, 0]),
        door(InnerDoorOpening3, [3, 0]),
        door(InnerDoorOpening4, [4, 0]),
        door(InnerDoorOpening5, [5, 0]),
        door(InnerDoorOpening6, [6, 0]),
        door(InnerDoorOpen, [7, 0]),

        door(OuterDoor, [0, 1]),
        door(OuterDoorOpening1, [1, 1]),
        door(OuterDoorOpening2, [2, 1]),
        door(OuterDoorOpening3, [3, 1]),
        door(OuterDoorOpening4, [4, 1]),
        door(OuterDoorOpening5, [5, 1]),
        door(OuterDoorOpening6, [6, 1]),
        door(OuterDoorOpen, [7, 1]),

        general_wall_fit(Window, [0, 0], None, None),

        feature(Light, [0, 0], None, None),
    ]
}

#[derive(Clone, Copy, Debug)]
pub struct InputSpriteLocation {
    pub position: Vector2<u32>,
    pub size: Vector2<u32>,
    pub offset: Vector2<i32>,
}

#[derive(Clone, Debug)]
pub enum InputSprite {
    Simple {
        sprite: Sprite,
        location: InputSpriteLocation,
    },
    Wall {
        sprite: Sprite,
        top: InputSpriteLocation,
        decorations: BTreeMap<Direction, Vector2<u32>>,
    },
    WallFit {
        sprite: Sprite,
        top: InputSpriteLocation,
        front: InputSpriteLocation,
    },
}

const WALL_START: Vector2<u32> = Vector2 { x: 0, y: 0 };
const WALL_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 22 };
const WALL_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 6 };
const WALL_DIRECTION_ORDER: [Direction; 8] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthWest,
];
const WALL_BLOCK_DIMENSIONS: Vector2<u32> = Vector2 {
    x: WALL_DIMENSIONS.x * 9,
    y: WALL_DIMENSIONS.y,
};
const WALL_TOTAL_HEIGHT: u32 = WALL_DIMENSIONS.y * 2;

const CHARACTER_START: Vector2<u32> = Vector2 { x: 0, y: WALL_START.y + WALL_TOTAL_HEIGHT };
const CHARACTER_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 20 };
const CHARACTER_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 8 };
const CHARACTER_TOTAL_HEIGHT: u32 = CHARACTER_DIMENSIONS.y * 1;

const FLOOR_START: Vector2<u32> = Vector2 { x: 0, y: CHARACTER_START.y + CHARACTER_TOTAL_HEIGHT };
const FLOOR_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 16 };
const FLOOR_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 0 };
const FLOOR_TOTAL_HEIGHT: u32 = FLOOR_DIMENSIONS.y * 2;

const DOOR_START: Vector2<u32> = Vector2 { x: 0, y: FLOOR_START.y + FLOOR_TOTAL_HEIGHT };
const DOOR_FRONT_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 22 };
const DOOR_TOP_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 22 };
const DOOR_FRONT_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 6 };
const DOOR_TOP_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 10 };
const DOOR_BLOCK_DIMENSIONS: Vector2<u32> = Vector2 { x: 32, y: 22 };
const DOOR_TOTAL_HEIGHT: u32 = DOOR_BLOCK_DIMENSIONS.y * 2;

const GENERAL_WALL_FIT_START: Vector2<u32> = Vector2 { x: 0, y: DOOR_START.y + DOOR_TOTAL_HEIGHT };
const GENERAL_WALL_FIT_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 16 };
const GENERAL_WALL_FIT_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 0 };
const GENERAL_WALL_FIT_TOTAL_HEIGHT: u32 = GENERAL_WALL_FIT_DIMENSIONS.y * 1;
const GENERAL_WALL_FIT_BLOCK_DIMENSIONS: Vector2<u32> = Vector2 { x: 32, y: 16 };

const FEATURE_START: Vector2<u32> = Vector2 { x: 0, y: GENERAL_WALL_FIT_START.y + GENERAL_WALL_FIT_TOTAL_HEIGHT };
const FEATURE_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 16 };
const FEATURE_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 0 };
const FEATURE_TOTAL_HEIGHT: u32 = FEATURE_DIMENSIONS.y * 1;

fn character(sprite: Sprite, position: [u32; 2], offset: Option<[i32; 2]>, size: Option<[u32; 2]>) -> InputSprite {
    let position = CHARACTER_START + Vector2::from(position).mul_element_wise(CHARACTER_DIMENSIONS);
    let offset = offset.map(Vector2::from).unwrap_or(CHARACTER_OFFSET);
    let size = size.map(Vector2::from).unwrap_or(CHARACTER_DIMENSIONS);
    InputSprite::Simple {
        sprite,
        location: InputSpriteLocation {
            position,
            size,
            offset,
        },
    }
}

fn floor(sprite: Sprite, position: [u32; 2], offset: Option<[i32; 2]>, size: Option<[u32; 2]>) -> InputSprite {
    let position = FLOOR_START + Vector2::from(position).mul_element_wise(FLOOR_DIMENSIONS);
    let offset = offset.map(Vector2::from).unwrap_or(FLOOR_OFFSET);
    let size = size.map(Vector2::from).unwrap_or(FLOOR_DIMENSIONS);
    InputSprite::Simple {
        sprite,
        location: InputSpriteLocation {
            position,
            size,
            offset,
        },
    }
}

fn wall(sprite: Sprite, position: [u32; 2], offset: Option<[i32; 2]>, size: Option<[u32; 2]>) -> InputSprite {
    let position = WALL_START + Vector2::from(position).mul_element_wise(WALL_BLOCK_DIMENSIONS);
    let offset = offset.map(Vector2::from).unwrap_or(WALL_OFFSET);
    let size = size.map(Vector2::from).unwrap_or(WALL_DIMENSIONS);

    let mut decorations = BTreeMap::new();
    for (index, direction) in WALL_DIRECTION_ORDER.iter().enumerate() {
        decorations.insert(*direction, position + Vector2::new(size.x * index as u32, 0));
    }
    let top = InputSpriteLocation {
        position: position + Vector2::new(size.x * WALL_DIRECTION_ORDER.len() as u32, 0),
        size,
        offset,
    };
    InputSprite::Wall {
        sprite,
        top,
        decorations,
    }
}

fn door(sprite: Sprite, position: [u32; 2]) -> InputSprite {
    let position = DOOR_START + Vector2::from(position).mul_element_wise(DOOR_BLOCK_DIMENSIONS);

    let front = InputSpriteLocation {
        position,
        size: DOOR_FRONT_DIMENSIONS,
        offset: DOOR_FRONT_OFFSET,
    };

    let top = InputSpriteLocation {
        position: position + Vector2::new(DOOR_FRONT_DIMENSIONS.x, 0),
        size: DOOR_TOP_DIMENSIONS,
        offset: DOOR_TOP_OFFSET,
    };

    InputSprite::WallFit { sprite, front, top }
}

fn general_wall_fit(sprite: Sprite, position: [u32; 2], offset: Option<[i32; 2]>, size: Option<[u32; 2]>) -> InputSprite {

    let position = GENERAL_WALL_FIT_START + Vector2::from(position).mul_element_wise(GENERAL_WALL_FIT_DIMENSIONS);
    let offset = offset.map(Vector2::from).unwrap_or(GENERAL_WALL_FIT_OFFSET);
    let size = size.map(Vector2::from).unwrap_or(GENERAL_WALL_FIT_DIMENSIONS);

    let front = InputSpriteLocation {
        position,
        size,
        offset,
    };

    let top = InputSpriteLocation {
        position: position + Vector2::new(size.x, 0),
        size,
        offset,
    };

    InputSprite::WallFit { sprite, front, top }
}

fn feature(sprite: Sprite, position: [u32; 2], offset: Option<[i32; 2]>, size: Option<[u32; 2]>) -> InputSprite {
    let position = FEATURE_START + Vector2::from(position).mul_element_wise(FEATURE_DIMENSIONS);
    let offset = offset.map(Vector2::from).unwrap_or(FEATURE_OFFSET);
    let size = size.map(Vector2::from).unwrap_or(FEATURE_DIMENSIONS);
    InputSprite::Simple {
        sprite,
        location: InputSpriteLocation {
            position,
            size,
            offset,
        },
    }
}
