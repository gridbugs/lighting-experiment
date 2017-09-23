use std::collections::HashMap;
use cgmath::Vector2;
use entity_store::{EntityId, EntityChange, EntityStore};
use spatial_hash::SpatialHashTable;
use content::ActionType;
use ai_info::GlobalAiInfo;
use append::Append;
use direction::CardinalDirections;
use static_grid::StaticGrid;
use search::PathNode;

#[derive(Debug)]
struct NpcInfo {
    id: EntityId,
    distance: u32,
    coord: Vector2<i32>,
}

pub struct AiEnv {
    npcs: Vec<NpcInfo>,
    movement_grid: StaticGrid<u64>,
    seq: u64,
    paths: HashMap<EntityId, Vec<PathNode>>,
}

impl AiEnv {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            npcs: Vec::new(),
            movement_grid: StaticGrid::new_copy(width, height, 0),
            seq: 0,
            paths: HashMap::new(),
        }
    }

    pub fn append_actions<A: Append<ActionType>>(&mut self,
                                                 actions: &mut A,
                                                 entity_store: &EntityStore,
                                                 spatial_hash: &SpatialHashTable,
                                                 global_info: &mut GlobalAiInfo)
    {
        self.seq += 1;
        self.npcs.clear();
        for id in entity_store.npc.iter() {
            let coord = if let Some(coord) = entity_store.coord.get(id).cloned() {
                coord
            } else {
                continue;
            };

            let distance = if let Some(distance) = global_info.get_distance(coord) {
                distance
            } else {
                continue;
            };

            self.npcs.push(NpcInfo {
                id: *id,
                coord,
                distance,
            });
        }
        self.npcs.sort_by(|a, b| {
            a.distance.cmp(&b.distance)
        });
        for npc in self.npcs.iter() {
            let remove_path = if let Some(path) = self.paths.get_mut(&npc.id) {
                if let Some(node) = path.pop() {
                    if node.origin == npc.coord {
                        actions.append(ActionType::Walk(npc.id, node.direction));
                        continue;
                    }
                }
                true
            } else {
                false
            };
            if remove_path {
                self.paths.remove(&npc.id);
            }

            let mut best_destination = None;
            let mut min_distance = ::std::u32::MAX;
            for direction in CardinalDirections {
                let destination = npc.coord + direction.vector();
                if let Some(distance) = global_info.get_distance(destination) {
                    if distance <= min_distance {
                        min_distance = distance;
                        best_destination = Some((direction, destination));
                    }
                }
            }

            if let Some((attempt_direction, destination)) = best_destination {

                if let Some(sh_cell) = spatial_hash.get_signed(destination) {
                    let coord = if sh_cell.player_count == 0 {
                        destination
                    } else {
                        npc.coord
                    };

                    let maybe_direction = if *self.movement_grid.get_checked(coord.cast()) == self.seq {
                        let mut path = Vec::new();
                        let result = global_info.search_to_player(spatial_hash, npc.coord, |sh_cell, coord| {
                            sh_cell.solid_count == 0 &&
                                sh_cell.door_set.is_empty() &&
                                *self.movement_grid.get_checked(coord) != self.seq
                        }, &mut path);
                        if result.is_ok() {
                            let first = path.pop().expect("Empty path");
                            self.paths.insert(npc.id, path);
                            Some(first.direction)
                        } else {
                            None
                        }
                    } else {
                        *self.movement_grid.get_checked_mut(coord.cast()) = self.seq;
                        Some(attempt_direction)
                    };

                    if let Some(direction) = maybe_direction {
                        actions.append(ActionType::Walk(npc.id, direction));
                    }
                }
            }
        }
    }

    pub fn update(&mut self, change: &EntityChange, entity_store: &EntityStore) {
        use self::EntityChange::*;
        match change {
            &Insert(id, _) => {
                if entity_store.player.contains(&id) {
                    self.paths.clear()
                }
            }
            _ => {}
        }
    }
}
