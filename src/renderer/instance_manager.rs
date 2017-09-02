use cgmath::Vector2;
use entity_store::{EntityStore, EntityChange,
                   ComponentValue, ComponentType, EntityVecMap};
use spatial_hash::SpatialHashTable;
use id_allocator::IdAllocator;

use renderer::tile_renderer::{Instance, SpriteRenderInfo};
use renderer::sprite_sheet::SpriteTable;

use direction::Directions;
use content::Sprite;

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

    fn update_sprite(&mut self, instances: &mut [Instance], spatial_hash: &SpatialHashTable, sprite_table: &SpriteTable,
                     index: InstanceIndex, position: Vector2<f32>, sprite: Sprite) {
        if let Some(sprite_info) = SpriteRenderInfo::resolve(
            sprite, sprite_table, position, spatial_hash
        ) {
            if let Some(wall_info) = sprite_info.wall_info {
                for (coord, dir) in izip!(spatial_hash.neighbour_coord_iter(position.cast(), Directions), Directions) {
                    if let Some(cell) = spatial_hash.get_valid(coord) {
                        for wall_id in cell.wall_set.iter() {
                            if let Some(index) = self.index_table.get(wall_id).cloned() {
                                let bitmap = cell.wall_neighbours.bitmap() | dir.opposite().bitmap();
                                let sprite_position = wall_info.position(bitmap.raw);
                                instances[index as usize].sprite_sheet_pix_coord = sprite_position.into();
                            }
                        }
                    }
                }
            }
            instances[index as usize].update_sprite_info(sprite_info);
        }
    }

    pub fn update(&mut self,
                  instances: &mut [Instance],
                  entity_change: &EntityChange,
                  entity_store: &EntityStore,
                  spatial_hash: &SpatialHashTable,
                  sprite_table: &SpriteTable) {

        use self::ComponentValue::*;
        use self::EntityChange::*;

        match entity_change {
            &Insert(id, Position(position)) => {
                let index = if let Some(index) = self.index_table.get(&id).cloned() {
                    index
                } else {
                    let index = self.index_allocator.allocate();
                    self.index_table.insert(id, index);
                    index
                };
                {
                    let instance = &mut instances[index as usize];
                    instance.position = position.into();
                    instance.enabled = 1;

                    if let Some(depth_type) = entity_store.depth.get(&id) {
                        instance.update_depth(position.y, spatial_hash.height() as f32, *depth_type);
                    }
                }

                if let Some(sprite) = entity_store.sprite.get(&id) {
                    self.update_sprite(instances, spatial_hash, sprite_table, index, position, *sprite);
                }
            }
            &Insert(id, Sprite(sprite)) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    if let Some(position) = entity_store.position.get(&id) {
                        self.update_sprite(instances, spatial_hash, sprite_table, index, *position, sprite);
                    }
                }
            }
            &Insert(id, Depth(depth_type)) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    if let Some(position) = entity_store.position.get(&id) {
                        instances[index as usize].update_depth(position.y, spatial_hash.height() as f32, depth_type);
                    }
                }
            }
            &Remove(id, ComponentType::Position) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    instances[index as usize].enabled = 0;
                    self.index_allocator.free(index);
                    self.index_table.remove(&id);
                }
            }
            &Remove(id, ComponentType::Sprite) => {
                if let Some(index) = self.index_table.get(&id).cloned() {
                    instances[index as usize].update_sprite_info(SpriteRenderInfo::blank());
                }
            }
            _ => {}
        }
    }
}
