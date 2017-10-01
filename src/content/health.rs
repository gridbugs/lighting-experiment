#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HealthInfo {
    pub current: u32,
    pub max: u32,
}

impl HealthInfo {
    pub fn full(max: u32) -> Self {
        Self {
            current: max,
            max,
        }
    }
}
