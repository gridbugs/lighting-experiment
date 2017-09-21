use entity_store::{EntityId, EntityStore};
use content::ActionType;
use ai_info::GlobalAiInfo;
use dijkstra_map::DirectionInfo;
use append::Append;
use direction::CardinalDirection;

#[derive(Debug)]
struct NpcInfo {
    id: EntityId,
    distance: u32,
    direction: CardinalDirection,
}

pub struct AiEnv {
    npcs: Vec<NpcInfo>,
}

impl AiEnv {
    pub fn new() -> Self {
        Self {
            npcs: Vec::new(),
        }
    }

    pub fn append_actions<A: Append<ActionType>>(&mut self, actions: &mut A, entity_store: &EntityStore, global_info: &GlobalAiInfo) {
        self.npcs.clear();
        for id in entity_store.npc.iter() {
            let coord = if let Some(coord) = entity_store.coord.get(id) {
                coord
            } else {
                continue;
            };

            let distance = if let Some(distance) = global_info.get_distance(*coord) {
                distance
            } else {
                continue;
            };

            let direction = if let Some(direction) = global_info.choose_direction(*coord).direction() {
                direction
            } else {
                continue;
            };

            self.npcs.push(NpcInfo {
                id: *id,
                distance,
                direction,
            });
        }
        self.npcs.sort_by(|a, b| {
            a.distance.cmp(&b.distance)
        });
        for npc in self.npcs.iter() {
            actions.append(ActionType::Walk(npc.id, npc.direction));
        }
    }
}

pub fn next_action(id: EntityId, entity_store: &EntityStore, global_info: &GlobalAiInfo) -> Option<ActionType> {

    let position = if let Some(position) = entity_store.position.get(&id) {
        *position
    } else {
        return None;
    };

    let info = global_info.choose_direction(position.cast());

    match info {
        DirectionInfo::NoInformation | DirectionInfo::AtDestination => None,
        DirectionInfo::Direction(direction) => {
            Some(ActionType::Walk(id, direction))
        }
    }
}
