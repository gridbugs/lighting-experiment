use cgmath::Vector2;
use entity_store::{EntityId, EntityStore};
use spatial_hash::SpatialHashTable;
use content::ActionType;
use ai_info::GlobalAiInfo;
use append::Append;
use direction::CardinalDirections;
use static_grid::StaticGrid;

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
}

impl AiEnv {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            npcs: Vec::new(),
            movement_grid: StaticGrid::new_copy(width, height, 0),
            seq: 0,
        }
    }

    pub fn append_actions<A: Append<ActionType>>(&mut self,
                                                 actions: &mut A,
                                                 entity_store: &EntityStore,
                                                 spatial_hash: &SpatialHashTable,
                                                 global_info: &GlobalAiInfo)
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
                    let seq = if sh_cell.player_count == 0 {
                        self.movement_grid.get_signed_mut(destination).expect("Incorrect movement grid size")
                    } else {
                        self.movement_grid.get_signed_mut(npc.coord).expect("Incorrect movement grid size")
                    };

                    let maybe_direction = if *seq == self.seq {
                        // TODO: try getting direction with search
                        None
                    } else {
                        *seq = self.seq;
                        Some(attempt_direction)
                    };

                    if let Some(direction) = maybe_direction {
                        actions.append(ActionType::Walk(npc.id, direction));
                    }
                }
            }
        }
    }
}
