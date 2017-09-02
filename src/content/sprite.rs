enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sprite {
    Blank = 0,
    Angler,
    AnglerBob,
    InnerFloor,
    OuterFloor,
    OuterWall,
    Door,
}
}

pub const NUM_SPRITES: usize = 7;
