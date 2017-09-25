use std::mem;
use entity_store::{EntityId, EntityChange, EntityStore, ComponentValue};
use content::ActionType;
use append::Append;
use turn::{TurnInfo, NUM_TURN_STATES};
use content::DoorState;

struct OpenDoor {
    id: EntityId,
    close_time: u64,
}

pub struct DoorManager {
    open_doors: Vec<OpenDoor>,
    open_doors_swap: Vec<OpenDoor>,
}

impl DoorManager {
    pub fn new() -> Self {
        Self {
            open_doors: Vec::new(),
            open_doors_swap: Vec::new(),
        }
    }

    pub fn update(&mut self, change: &EntityChange, turn: TurnInfo) {
        use self::EntityChange::*;
        match change {
            &Insert(id, ComponentValue::Door(door_info)) => {
                if door_info.state == DoorState::Open {
                    self.open_doors.push(OpenDoor {
                        id,
                        close_time: turn.count + NUM_TURN_STATES as u64 + 1,
                    });
                }
            }
            _ => {}
        }
    }

    pub fn close_doors<A: Append<ActionType>>(&mut self,
                                              actions: &mut A,
                                              entity_store: &EntityStore,
                                              turn: TurnInfo)
    {
        for door in self.open_doors.drain(..) {
            if let Some(door_info) = entity_store.door.get(&door.id) {
                if door_info.state == DoorState::Open {
                    if turn.count >= door.close_time {
                        actions.append(ActionType::CloseDoor(door.id));
                    }
                    self.open_doors_swap.push(door);
                }
            }
        }
        mem::swap(&mut self.open_doors, &mut self.open_doors_swap);
    }
}
