enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sprite {
    Blank = 0,
    Angler,
    AnglerBob,
    OuterFloor,
    InnerFloor,
    OuterWall,
    InnerWall,
    OuterDoor,
    InnerDoor,
    Window,
}
}

pub const NUM_SPRITES: usize = 10;
