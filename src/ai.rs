use entity_store::{EntityId, EntityStore};
use content::ActionType;
use ai_info::GlobalAiInfo;
use dijkstra_map::DirectionInfo;

pub fn next_action(id: EntityId, entity_store: &EntityStore, global_info: &GlobalAiInfo) -> Option<ActionType> {

    let position = if let Some(position) = entity_store.position.get(&id) {
        *position
    } else {
        return None;
    };

    let info = if entity_store.door_opener.contains(&id) {
        global_info.choose_direction_doors(position.cast())
    } else {
        global_info.choose_direction_no_doors(position.cast())
    };

    match info {
        DirectionInfo::NoInformation | DirectionInfo::AtDestination => None,
        DirectionInfo::Direction(direction) => {
            Some(ActionType::Walk(id, direction))
        }
    }
}
