#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileSprite {
    Blank = 0,

    Angler,
    Crab,
    Snail,

    OuterFloor,
    InnerFloor,
    OuterWall,
    InnerWall,
    InnerWater,

    InnerDoor,
    InnerDoorOpening1,
    InnerDoorOpening2,
    InnerDoorOpening3,
    InnerDoorOpening4,
    InnerDoorOpening5,
    InnerDoorOpening6,
    InnerDoorOpen,

    OuterDoor,
    OuterDoorOpening1,
    OuterDoorOpening2,
    OuterDoorOpening3,
    OuterDoorOpening4,
    OuterDoorOpening5,
    OuterDoorOpening6,
    OuterDoorOpen,

    Window,

    Light,

    _Num,
}

pub const NUM_TILE_SPRITES: usize = TileSprite::_Num as usize;
