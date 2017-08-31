use std::collections::VecDeque;
use frontend::{Frontend, FrontendOutput, FrontendInput, OutputWorldState};
use terrain::demo;
use entity_store::{EntityStore, ComponentValue, EntityChange};
use spatial_hash::SpatialHashTable;
use entity_id_allocator::EntityIdAllocator;
use content::ActionType;
use control_table::GameControlTable;
use control::Control;
use input::{Input, Bindable, Unbindable, System};
use direction::CardinalDirection;

pub fn launch<I: FrontendInput, O: for<'a> FrontendOutput<'a>>(frontend: Frontend<I, O>) {
    let Frontend { input: mut frontend_input, output: mut frontend_output } = frontend;
    let control_table = {
        use self::Bindable::*;
        use self::Control::*;
        use self::CardinalDirection::*;
        GameControlTable::new(hashmap!{
            Up => Move(North),
            Right => Move(East),
            Down => Move(South),
            Left => Move(West),
        })
    };

    let mut allocator = EntityIdAllocator::new();
    let mut changes = Vec::new();
    let mut entity_store = EntityStore::new();

    let metadata = demo::generate(&mut changes, &mut allocator);
    let player_id = metadata.player_id.expect("No player");

    let mut spatial_hash = SpatialHashTable::new(metadata.width, metadata.height);

    frontend_output.with_world_state(|state| {
        for c in changes.drain(..) {
            state.update(&c, &entity_store, &spatial_hash);
            spatial_hash.update(&entity_store, &c, 0);

            if let EntityChange::Insert(id, ComponentValue::Position(new_position)) = c {
                if id == player_id {
                    state.set_player_position(new_position);
                }
            }

            entity_store.commit(c);
        }
    });

    let mut proposed_actions = VecDeque::new();
    let mut staged_changes = VecDeque::new();

    let mut running = true;
    let mut count = 0;
    while running {

        frontend_input.with_input(|input| {
            use self::Input::*;
            match input {
                Bindable(b) => {
                    if let Some(control) = control_table.get(b) {
                        use self::Control::*;
                        match control {
                            Move(direction) => {
                                proposed_actions.push_back(ActionType::Walk(player_id, direction));
                            }
                        }
                    }
                }
                Unbindable(u) => {
                    use self::Unbindable::*;
                    match u {
                        Escape => {}
                    }
                }
                System(s) => {
                    use self::System::*;
                    match s {
                        Quit => running = false,
                        Resize(w, h) => {
                            frontend_output.handle_resize(w, h);
                        }
                    }
                }
            }
        });

        if count % 30 == 0 {
            proposed_actions.push_back(ActionType::Bob(player_id));
        }

        for a in proposed_actions.drain(..) {
            a.populate(&entity_store, &mut staged_changes);
        }

        frontend_output.with_world_state(|state| {

            for change in staged_changes.drain(..) {

                state.update(&change, &entity_store, &spatial_hash);

                spatial_hash.update(&entity_store, &change, count);

                if let EntityChange::Insert(id, ComponentValue::Position(new_position)) = change {
                    if id == player_id {
                        state.set_player_position(new_position);
                    }
                }

                entity_store.commit(change);
            }
        });

        frontend_output.draw();

        count += 1;
    }
}
