#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriteEffect {
    OuterWater,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpriteEffectInfo {
    pub effect: SpriteEffect,
    pub args: [f32; 4],
}

impl SpriteEffectInfo {
    pub fn new(effect: SpriteEffect) -> Self {
        Self {
            effect,
            args: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn with_args(effect: SpriteEffect, args: [f32; 4]) -> Self {
        Self { effect, args }
    }
}
