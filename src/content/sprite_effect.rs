#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriteEffect {
    Water,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpriteEffectInfo {
    pub effect: SpriteEffect,
    pub args: [f32; 4],
}

impl SpriteEffectInfo {
    pub fn water(steps: u32, min: f32, max: f32) -> Self {
        Self {
            effect: SpriteEffect::Water,
            args: [steps as f32, min, max, 0.0],
        }
    }
}
