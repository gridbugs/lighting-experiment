enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sprite {
    Angler,
    InnerFloor,
    OuterFloor,
    OuterWall,
}
}

pub const NUM_SPRITES: usize = 4;
