use entity_store::EntityId;

pub struct TerrainMetadata {
    pub player_id: Option<EntityId>,
    pub width: u32,
    pub height: u32,
}

impl Default for TerrainMetadata {
    fn default() -> Self {
        TerrainMetadata {
            player_id: None,
            width: 0,
            height: 0,
        }
    }
}
