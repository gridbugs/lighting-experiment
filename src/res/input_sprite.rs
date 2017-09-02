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
        character(Angler, CHARACTER_START + Vector2::new(0, 0).mul_element_wise(CHARACTER_DIMENSIONS)),
        floor(InnerFloor, FLOOR_START + Vector2::new(0, 0).mul_element_wise(FLOOR_DIMENSIONS)),
        floor(OuterFloor, FLOOR_START + Vector2::new(0, 1).mul_element_wise(FLOOR_DIMENSIONS)),
        wall(OuterWall, WALL_START + Vector2::new(0, 0).mul_element_wise(WALL_BLOCK_DIMENSIONS)),
        door(Door, DOOR_START + Vector2::new(0, 0).mul_element_wise(DOOR_BLOCK_DIMENSIONS)),
        character(AnglerBob, CHARACTER_START + Vector2::new(1, 0).mul_element_wise(CHARACTER_DIMENSIONS)),
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
    Door {
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
    x: WALL_DIMENSIONS.x,
    y: WALL_DIMENSIONS.y * 9,
};

const CHARACTER_START: Vector2<u32> = Vector2 { x: 0, y: 22 };
const CHARACTER_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 20 };
const CHARACTER_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 8 };

const FLOOR_START: Vector2<u32> = Vector2 { x: 0, y: 42 };
const FLOOR_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 16 };
const FLOOR_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 0 };

const DOOR_START: Vector2<u32> = Vector2 { x: 0, y: 74 };
const DOOR_FRONT_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 22 };
const DOOR_TOP_DIMENSIONS: Vector2<u32> = Vector2 { x: 16, y: 22 };
const DOOR_FRONT_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 6 };
const DOOR_TOP_OFFSET: Vector2<i32> = Vector2 { x: 0, y: 10 };
const DOOR_BLOCK_DIMENSIONS: Vector2<u32> = Vector2 { x: 32, y: 22 };

fn character(sprite: Sprite, position: Vector2<u32>) -> InputSprite {
    InputSprite::Simple {
        sprite,
        location: InputSpriteLocation {
            position: position,
            size: CHARACTER_DIMENSIONS,
            offset: CHARACTER_OFFSET,
        },
    }
}

fn floor(sprite: Sprite, position: Vector2<u32>) -> InputSprite {
    InputSprite::Simple {
        sprite,
        location: InputSpriteLocation {
            position: position,
            size: FLOOR_DIMENSIONS,
            offset: FLOOR_OFFSET,
        },
    }
}

fn wall(sprite: Sprite, position: Vector2<u32>) -> InputSprite {
    let mut decorations = BTreeMap::new();
    for (index, direction) in WALL_DIRECTION_ORDER.iter().enumerate() {
        decorations.insert(*direction, position + Vector2::new(WALL_DIMENSIONS.x * index as u32, 0));
    }
    let top = InputSpriteLocation {
        position: position + Vector2::new(WALL_DIMENSIONS.x * WALL_DIRECTION_ORDER.len() as u32, 0),
        size: WALL_DIMENSIONS,
        offset: WALL_OFFSET,
    };
    InputSprite::Wall {
        sprite,
        top,
        decorations,
    }
}

fn door(sprite: Sprite, position: Vector2<u32>) -> InputSprite {
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

    InputSprite::Door { sprite, front, top }
}
