#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DepthType {
    Bottom,
    Fixed,
    Gradient,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DepthInfo {
    pub typ: DepthType,
    pub offset: f32,
}

impl From<DepthType> for DepthInfo {
    fn from(typ: DepthType) -> Self {
        Self {
            typ,
            offset: 0.0,
        }
    }
}

impl DepthInfo {
    pub fn new(typ: DepthType, offset: f32) -> Self {
        Self { typ, offset }
    }
}
