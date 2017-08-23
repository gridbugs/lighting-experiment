use entity_store::{EntityStore, EntityChange, Change, EntityId,
                   ComponentValue, ComponentType, EntityVecMap};
use spatial_hash::SpatialHashTable;
use id_allocator::IdAllocator;

use renderer::tile_renderer::{Instance, SpriteRenderInfo};
use renderer::sprite_sheet::SpriteTable;

use content::DepthType;

type InstanceIndex = u16;

pub struct InstanceManager {
    index_allocator: IdAllocator<InstanceIndex>,
    index_table: EntityVecMap<InstanceIndex>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            index_allocator: IdAllocator::new(),
            index_table: EntityVecMap::new(),
        }
    }

    pub fn num_instances(&self) -> u32 {
        self.index_allocator.max() as u32
    }

    pub fn index(&mut self, id: EntityId) -> usize {
        if let Some(index) = self.index_table.get(&id).cloned() {
            index as usize
        } else {
            let index = self.index_allocator.allocate();
            self.index_table.insert(id, index);
            index as usize
        }
    }

    pub fn update(&mut self,
                  instances: &mut [Instance],
                  entity_change: &EntityChange,
                  entity_store: &EntityStore,
                  spatial_hash: &SpatialHashTable,
                  sprite_table: &SpriteTable) {

        use self::ComponentValue::*;
        use self::Change::*;

        let id = entity_change.id;

        match &entity_change.change {
            &Insert(Position(position)) => {
                let index = if let Some(index) = self.index_table.get(&id).cloned() {
                    index
                } else {
                    let index = self.index_allocator.allocate();
                    self.index_table.insert(id, index);
                    index
                };
                let instance = &mut instances[index as usize];
                instance.position = position.into();
                instance.enabled = 1;

                if let Some(sprite) = entity_store.sprite.get(&id) {
                    if let Some(sprite_info) = SpriteRenderInfo::resolve(
                        *sprite, sprite_table, position, spatial_hash
                    ) {
                        instance.update_sprite_info(sprite_info);
                    }
                }

                if let Some(depth_type) = entity_store.depth.get(&id) {
                    let depth = match *depth_type {
                        DepthType::Vertical => 1.0 - position.x / spatial_hash.height() as f32,
                        DepthType::Horizontal => 1.0,
                    };
                    instance.depth = depth;
                }
            }
            &Insert(Sprite(sprite)) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    if let Some(position) = entity_store.position.get(&id) {
                        if let Some(sprite_info) = SpriteRenderInfo::resolve(
                            sprite, sprite_table, *position, spatial_hash
                        ) {
                            instances[index as usize].update_sprite_info(sprite_info);
                        }
                    }
                }
            }
            &Insert(Depth(depth_type)) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    if let Some(position) = entity_store.position.get(&id) {
                        let depth = match depth_type {
                            DepthType::Vertical => 1.0 - position.x / spatial_hash.height() as f32,
                            DepthType::Horizontal => 1.0,
                        };
                        instances[index as usize].depth = depth;
                    }
                }
            }
            &Remove(ComponentType::Position) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    instances[index as usize].enabled = 0;
                    self.index_allocator.free(index);
                    self.index_table.remove(&id);
                }
            }
            &Remove(ComponentType::Sprite) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    instances[index as usize].update_sprite_info(SpriteRenderInfo::blank());
                }
            }
            _ => {}
        }
    }
}