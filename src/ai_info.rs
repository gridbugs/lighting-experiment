use cgmath::Vector2;
use dijkstra_map::DijkstraMap;
use search::{self, SearchEnv, PathNode};
use entity_store::{EntityChange, ComponentValue, EntityStore};
use spatial_hash::{SpatialHashTable, SpatialHashCell};

const DISTANCE_TO_PLAYER_THRESHOLD: u32 = 20;

pub struct GlobalAiInfo {
    distance_to_player: DijkstraMap,
    search_env: SearchEnv,
    player_coord: Vector2<i32>,
}

fn general_can_enter(cell: &SpatialHashCell) -> bool {
    cell.solid_count == 0 && cell.door_set.is_empty()
}

impl GlobalAiInfo {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            distance_to_player: DijkstraMap::new(width, height),
            search_env: SearchEnv::new(width, height),
            player_coord: Vector2::new(0, 0),
        }
    }

    pub fn set_player_coord(&mut self, coord: Vector2<i32>, spatial_hash: &SpatialHashTable) {
        self.player_coord = coord;
        self.distance_to_player.compute_distance_to_coord(spatial_hash, coord, DISTANCE_TO_PLAYER_THRESHOLD, general_can_enter);
    }

    pub fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {
        use self::EntityChange::*;
        use self::ComponentValue::*;
        match change {
            &Insert(id, Coord(coord)) => {
                if entity_store.player.contains(&id) {
                    self.set_player_coord(coord, spatial_hash);
                }
            }
            _ => {}
        }
    }

    pub fn get_distance(&self, coord: Vector2<i32>) -> Option<u32> {
        self.distance_to_player.get_distance(coord)
    }

    pub fn search_to_player<P>(&mut self,
                               spatial_hash: &SpatialHashTable,
                               start: Vector2<i32>,
                               can_enter: P,
                               path: &mut Vec<PathNode>) -> search::Result<()>
        where P: Fn(&SpatialHashCell, Vector2<u32>) -> bool,
    {
        self.search_env.search(spatial_hash, start, self.player_coord, can_enter, path)
    }
}
