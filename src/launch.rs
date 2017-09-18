use std::collections::VecDeque;
use std::time::Instant;
use std::mem;
use cgmath::Vector2;
use frontend::{FrontendOutput, FrontendInput, OutputWorldState, LightUpdate};
use terrain;
use entity_store::{EntityStore, ComponentValue, EntityChange};
use spatial_hash::SpatialHashTable;
use entity_id_allocator::EntityIdAllocator;
use content::ActionType;
use control_table::GameControlTable;
use control::Control;
use input::{Input, Bindable, Unbindable, System};
use direction::CardinalDirection;
use content::{ChangeDesc, Animation, AnimationStatus, AnimatedChange};
use policy;
use vision::shadowcast;

pub fn launch<I: FrontendInput, O: for<'a> FrontendOutput<'a>>(mut frontend_input: I, mut frontend_output: O) {
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

    let metadata = terrain::demo::generate(&mut changes, &mut allocator);
    let player_id = metadata.player_id.expect("No player");

    let mut spatial_hash = SpatialHashTable::new(metadata.width, metadata.height);
    let mut shadowcast_env = shadowcast::ShadowcastEnv::new();

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

    let mut animations: VecDeque<Animation> = VecDeque::new();
    let mut animations_swap = VecDeque::new();
    let mut animated_changes = VecDeque::new();

    let mut running = true;
    let mut count = 1;

    let start_instant = Instant::now();
    let mut frame_instant = start_instant;

    while running {

        let now = Instant::now();
        let frame_duration = now - frame_instant;
        let total_duration = now - start_instant;
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

        let visible_range = frontend_output.visible_range();

        for a in proposed_actions.drain(..) {
            a.populate(&entity_store, &mut change_descs);
        }

        for animation in animations.drain(..) {
            let status = animation.populate(frame_duration, &mut animated_changes);
            match status {
                AnimationStatus::Running(animation) => {
                    animations_swap.push_back(animation);
                }
                AnimationStatus::Finished => {}
            }
        }
        mem::swap(&mut animations, &mut animations_swap);

        for animated_change in animated_changes.drain(..) {
            match animated_change {
                AnimatedChange::Checked(change) => {
                    if policy::check(&change, &entity_store, &spatial_hash, &mut change_descs) {
                        staged_changes.push_back(change);
                    }
                }
                AnimatedChange::Unchecked(change) => staged_changes.push_back(change),
            }
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

        frontend_output.with_world_state(|state| {

            state.set_frame_info(count, total_duration);

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

            for (id, light_info) in entity_store.light.iter() {
                if let Some(position) = entity_store.position.get(id) {
                    if let Some((mut light_grid, light_update)) = state.next_light() {
                        shadowcast::observe(&mut light_grid, &mut shadowcast_env, *position, &spatial_hash,
                                            light_info.range, visible_range, count);
                        light_update.set_position(*position + Vector2::new(0.5, 0.5));
                        light_update.set_height(light_info.height);
                        light_update.set_intensity(light_info.intensity);
                        light_update.set_colour(light_info.colour);
                    }
                }
            }

            if let Some(player_position) = entity_store.position.get(&player_id) {
                shadowcast::observe(&mut state.vision_grid(), &mut shadowcast_env, *player_position, &spatial_hash,
                                    8, visible_range, count);
            }
        });

        frontend_output.draw();

        count += 1;
    }
}
