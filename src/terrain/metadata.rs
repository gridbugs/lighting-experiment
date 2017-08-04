use entity_store::EntityId;

pub struct TerrainMetadata {
    pub player_id: Option<EntityId>,
}

impl Default for TerrainMetadata {
    fn default() -> Self {
        TerrainMetadata {
            player_id: None,
        }
    }
}
