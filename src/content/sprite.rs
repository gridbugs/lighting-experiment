#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sprite {
    Blank = 0,
    Angler,
    AnglerBob,
    OuterFloor,
    InnerFloor,
    OuterWall,
    InnerWall,

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
    _Num,
}

pub const NUM_SPRITES: usize = Sprite::_Num as usize;
