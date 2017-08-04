enum_from_primitive! {
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Sprite {
    Angler,
    InnerFloor,
    OuterFloor,
    OuterWall,
}
}

pub const NUM_SPRITES: usize = 4;
