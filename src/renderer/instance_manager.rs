use std::collections::BinaryHeap;
use fnv;

use frame_env::FrameEnv;
use entity_store::{EntityId, DataChangeType};
use renderer::tile_renderer::Instance;

pub struct InstanceManager {
    indices: fnv::FnvHashMap<EntityId, usize>,
    free_indices: BinaryHeap<usize>,
    max_num_indices: usize,
}

impl InstanceManager {
    pub fn new(max_num_indices: usize) -> Self {
        InstanceManager {
            indices: fnv::FnvHashMap::default(),
            free_indices: (0..max_num_indices).collect(),
            max_num_indices,
        }
    }

    pub fn update(&mut self,
                  instances: &mut [Instance],
                  env: &FrameEnv) {

        for (id, change) in env.change.position.iter() {
            match change {
                &DataChangeType::Insert(position) => {
                    if let Some(index) = self.indices.get(id).map(|i| *i) {
                        // entity moves
                        instances[index].coord = position.into();
                    } else {
                        // entity gains position
                        let index = self.free_indices.pop()
                            .expect("Out of indices");
                        self.indices.insert(*id, index);
                        let sprite = post_change_get!(
                            &env.entity_store,
                            &env.change,
                            *id,
                            sprite);
                        // TODO create instance
                        instances[index] = unimplemented!();
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
                        instances[*index].sprite_index = unimplemented!();
                    }
                    &DataChangeType::Remove => {
                        // entity loses sprite
                        instances[*index].depth = -1.0; // hides instance
                    }
                }
            }
        }
    }
}
