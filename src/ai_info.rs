use cgmath::Vector2;
use dijkstra_map::{DijkstraMap, DirectionInfo};
use entity_store::{EntityChange, ComponentValue, EntityStore};
use spatial_hash::{SpatialHashTable, SpatialHashCell};

const DISTANCE_TO_PLAYER_THRESHOLD: u32 = 20;

pub struct GlobalAiInfo {
    distance_to_player_no_doors: DijkstraMap,
    distance_to_player_doors: DijkstraMap,
}

fn can_enter_no_doors(cell: &SpatialHashCell) -> bool {
    cell.solid_count == 0
}

fn can_enter_doors(cell: &SpatialHashCell) -> bool {
    cell.solid_count == 0 && cell.door_set.is_empty()
}

impl GlobalAiInfo {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            distance_to_player_no_doors: DijkstraMap::new(width, height),
            distance_to_player_doors: DijkstraMap::new(width, height),
        }
    }

    pub fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {
        use self::EntityChange::*;
        use self::ComponentValue::*;
        match change {
            &Insert(id, Position(position)) => {
                if entity_store.player.contains(&id) {
                    let position = position.cast();
                    self.distance_to_player_no_doors
                        .compute_distance_to_coord(spatial_hash, position, DISTANCE_TO_PLAYER_THRESHOLD, can_enter_no_doors);
                    self.distance_to_player_doors
                        .compute_distance_to_coord(spatial_hash, position, DISTANCE_TO_PLAYER_THRESHOLD, can_enter_doors);
                }
            }
            _ => {}
        }
    }

    pub fn choose_direction_doors(&self, coord: Vector2<u32>) -> DirectionInfo {
        self.distance_to_player_doors.choose_direction(coord)
    }

    pub fn choose_direction_no_doors(&self, coord: Vector2<u32>) -> DirectionInfo {
        self.distance_to_player_no_doors.choose_direction(coord)
    }
}
