use frontend::{Frontend, FrontendOutput, FrontendInput, OutputWorldState};
use terrain::demo;
use entity_store::{EntityStore, insert};
use spatial_hash::SpatialHashTable;
use entity_id_allocator::EntityIdAllocator;
use content::Sprite;
use control_table::GameControlTable;
use control::Control;
use input::{Input, Bindable, Unbindable, System};
use direction::CardinalDirection;

pub fn launch<I: FrontendInput, O: for<'a> FrontendOutput<'a>>(frontend: Frontend<I, O>) {
    let Frontend { mut input, mut output } = frontend;
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

    output.with_world_state(|state| {
        for c in changes.drain(..) {
            state.update(&c, &entity_store, &spatial_hash);
            spatial_hash.update(&entity_store, &c, 0);
            entity_store.commit(c);
        }
    });

    let mut running = true;
    let mut count = 0;
    while running {

        input.with_input(|input| {
            use self::Input::*;
            match input {
                Bindable(b) => {
                    if let Some(control) = control_table.get(b) {
                        use self::Control::*;
                        match control {
                            Move(direction) => {
                                println!("Moving {:?}", direction);
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
                            output.handle_resize(w, h);
                        }
                    }
                }
            }
        });

        output.with_world_state(|state| {

            if count % 45 == 0 {
                let change = if count % 90 == 0 {
                    insert::sprite(player_id, Sprite::Angler)
                } else {
                    insert::sprite(player_id, Sprite::AnglerBob)
                };

                state.update(&change, &entity_store, &spatial_hash);

                let player_position = entity_store.position.get(&player_id).cloned().expect("Failed to find player position");
                state.set_player_position(player_position);

                spatial_hash.update(&entity_store, &change, count);
                entity_store.commit(change);
            }
        });

        output.draw();

        count += 1;
    }
}
