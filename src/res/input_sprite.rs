use std::collections::BTreeMap;
use cgmath::Vector2;
use direction::{self, Direction};
use content::Sprite;

pub const WIDTH_PX: u32 = 32;
pub const HEIGHT_PX: u32 = 32;
pub const DIMENSIONS: Vector2<u32> = Vector2 { x: WIDTH_PX, y: HEIGHT_PX };

#[derive(Clone, Debug)]
pub enum InputSpriteCoord {
    Simple(Sprite, (u32, u32)),
    Wall(Sprite, (u32, u32), BTreeMap<Direction, (u32, u32)>),
}

pub fn input_sprite_coords() -> Vec<InputSpriteCoord> {

    use self::InputSpriteCoord::*;
    use self::Sprite::*;
    use self::Direction::*;

    vec![
        Simple(Angler, (0, 1)),
        Simple(InnerFloor, (0, 2)),
        Simple(OuterFloor, (0, 3)),
        Wall(OuterWall, (8, 0), btreemap!{
            North => (0, 0),
            East => (1, 0),
            South => (2, 0),
            West => (3, 0),
            NorthEast => (4, 0),
            SouthEast => (5, 0),
            SouthWest => (6, 0),
            NorthWest => (7, 0),
        }),
    ]
}

#[derive(Clone, Debug)]
pub enum InputSpritePixelCoord {
    Simple {
        sprite: Sprite,
        coord: Vector2<u32>,
    },
    Wall {
        sprite: Sprite,
        top: Vector2<u32>,
        decorations: BTreeMap<Direction, Vector2<u32>>,
    },
}

impl<'a> From<&'a InputSpriteCoord> for InputSpritePixelCoord {
    fn from(s: &'a InputSpriteCoord) -> Self {
        match s {
            &InputSpriteCoord::Simple(sprite, (x, y)) => {
                InputSpritePixelCoord::Simple {
                    sprite: sprite,
                    coord: Vector2::new(x * WIDTH_PX, y * HEIGHT_PX),
                }
            }
            &InputSpriteCoord::Wall(sprite, (x, y), ref decorations) => {

                let decorations: BTreeMap<Direction, Vector2<u32>> = decorations.iter()
                    .map(|(dir, &(x, y))| {
                        (*dir, Vector2::new(x * WIDTH_PX, y * HEIGHT_PX))
                    }).collect();

                assert_eq!(decorations.len(), direction::NUM_DIRECTIONS,
                    "Incomplete direction table in sprite sheet!");

                InputSpritePixelCoord::Wall {
                    sprite: sprite,
                    top: Vector2::new(x * WIDTH_PX, y * HEIGHT_PX),
                    decorations: decorations,
                }
            }
        }
    }
}

pub fn input_sprite_pixel_coords() -> Vec<InputSpritePixelCoord> {
    input_sprite_coords().iter().map(InputSpritePixelCoord::from).collect()
}
