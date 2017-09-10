#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightInfo {
    pub intensity: f32,
    pub range: u32,
    pub height: f32,
    pub colour: [f32; 3],
}

impl LightInfo {
    pub fn new(intensity: f32, range: u32, height: f32,
               r: f32, g: f32, b: f32) -> Self {
        Self {
            intensity,
            range,
            height,
            colour: [r, g, b],
        }
    }
}
