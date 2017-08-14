use fnv;

use frame_env::FrameEnv;
use entity_store::{EntityId, DataChangeType};
use renderer::tile_renderer::Instance;
use renderer::sprite_sheet::SpriteTable;
/*
pub struct InstanceManager {
    indices: fnv::FnvHashMap<EntityId, usize>,
    free_indices: Vec<usize>,
    num_indices: usize,
}

impl InstanceManager {
    pub fn new(max_num_indices_hint: Option<usize>) -> Self {
        InstanceManager {
            indices: fnv::FnvHashMap::default(),
            free_indices: Vec::with_capacity(max_num_indices_hint.unwrap_or(1)),
            num_indices: 0,
        }
    }

    pub fn update(&mut self,
                  instances: &mut [Instance],
                  sprite_table: &SpriteTable,
                  env: &FrameEnv) {

        for (id, change) in env.change.position.iter() {
            match change {
                &DataChangeType::Insert(position) => {
                    if let Some(index) = self.indices.get(id).map(|i| *i) {
                        // entity moves
                        instances[index].coord = position.into();
                    } else {
                        // entity gains position
                        let index = if let Some(index) = self.free_indices.pop() {
                            index
                        } else {
                            let index = self.num_indices;
                            self.num_indices += 1;
                            index
                        };

                        self.indices.insert(*id, index);
                        let sprite = post_change_get!(
                            &env.entity_store,
                            &env.change,
                            *id,
                            sprite);
                        // TODO create instance
                        instances[index] = Instance {
                            sprite_index: 0.0,
                            size: [32.0, 32.0],
                            coord: position.into(),
                            depth: 1.0,
                        };
                    };
                }
                &DataChangeType::Remove => {
                    // entity loses position
                    if let Some(index) = self.indices.remove(id) {
                        self.free_indices.push(index);
                        instances[index].depth = -1.0; // hides instance
                    }
                }
            }
        }

        for (id, change) in env.change.sprite.iter() {
            if let Some(index) = self.indices.get(id) {
                match change {
                    &DataChangeType::Insert(sprite) => {
                        // entity gains or changes sprite
                        // TODO add depth to sprite type
                        instances[*index].sprite_index = 0.0;
                    }
                    &DataChangeType::Remove => {
                        // entity loses sprite
                        instances[*index].depth = -1.0; // hides instance
                    }
                }
            }
        }
    }
}*/
