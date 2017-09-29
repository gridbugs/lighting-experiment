#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthOverlay {
    Full,
    Half,
    Empty,

    _Num,
}

pub const NUM_HEALTH_OVERLAYS: usize = HealthOverlay::_Num as usize;
