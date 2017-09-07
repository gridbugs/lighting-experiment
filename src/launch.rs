use std::collections::VecDeque;
use std::time::{Instant, Duration};
use std::mem;

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
use content::{ChangeDesc, AnimationStatus};
use policy;
use vision::{VisionCell, square};
use static_grid::StaticGrid;

struct VCell {
    last_seen: u64,
}

impl VisionCell for VCell {
    fn see(&mut self, time: u64) {
        self.last_seen = time;
    }
}

impl Default for VCell {
    fn default() -> Self {
        Self {
            last_seen: 0,
        }
    }
}

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
    let mut vision_grid: StaticGrid<VCell> = StaticGrid::new_default(metadata.width, metadata.height);

    frontend_output.update_world_size(metadata.width, metadata.height);

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

    let mut change_descs = VecDeque::new();
    let mut change_descs_swap = VecDeque::new();

    let mut animations = VecDeque::new();
    let mut animations_swap = VecDeque::new();

    let mut running = true;
    let mut count = 0;

    let bob_duration = Duration::from_millis(500);
    let mut bob_acc = Duration::from_millis(0);

    let mut frame_instant = Instant::now();

    while running {

        let now = Instant::now();
        let frame_duration = now - frame_instant;
        frame_instant = now;

        frontend_input.with_input(|input| {
            use self::Input::*;
            match input {
                Bindable(b) => {
                    if !animations.is_empty() {
                        return;
                    }
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

        bob_acc += frame_duration;
        if bob_acc >= bob_duration {
            bob_acc -= bob_duration;
            proposed_actions.push_back(ActionType::Bob(player_id));
        }

        for a in proposed_actions.drain(..) {
            a.populate(&entity_store, &mut change_descs);
        }

        loop {
            for desc in change_descs.drain(..) {
                use self::ChangeDesc::*;
                match desc {
                    Immediate(change) => {
                        if policy::check(&change, &entity_store, &spatial_hash, &mut change_descs_swap) {
                            staged_changes.push_back(change);
                        }
                    }
                    AnimatedChange(eventual_change, animation) => {
                        if policy::check(&eventual_change, &entity_store, &spatial_hash, &mut change_descs_swap) {
                            animations.push_back(animation);
                        }
                    }
                    Animation(animation) => {
                        animations.push_back(animation);
                    }
                }
            }
            if change_descs_swap.is_empty() {
                break;
            } else {
                mem::swap(&mut change_descs, &mut change_descs_swap);
            }
        }

        for mut animation in animations.drain(..) {
            let status = animation.populate(frame_duration, &mut staged_changes);
            if status == AnimationStatus::Running {
                animations_swap.push_back(animation);
            }
        }
        mem::swap(&mut animations, &mut animations_swap);

        frontend_output.with_world_state(|state| {

            state.set_frame_info(count);

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

            let player_position = entity_store.position.get(&player_id).expect("No player position");
            square::observe(&mut vision_grid, *player_position, &spatial_hash, count);
        });

        frontend_output.draw();

        count += 1;
    }
}
