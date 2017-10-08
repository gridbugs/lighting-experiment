#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HealthInfo {
    pub current: i32,
    pub max: i32,
}

impl HealthInfo {
    pub fn full(max: i32) -> Self {
        Self {
            current: max,
            max,
        }
    }
    pub fn reduce(self, amount: i32) -> Self {
        Self {
            current: self.current - amount,
            max: self.max,
        }
    }
}
