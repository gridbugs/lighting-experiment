use std::time::Duration;
use cgmath::Vector2;

use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;
use vision::VisionCell;
use grid::GridMut;

use input::Input;

pub trait OutputWorldState<'a, 'b> {
    type VisionCellType: VisionCell;
    type VisionCellGrid: GridMut<Self::VisionCellType>;
    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable);
    fn set_player_position(&mut self, player_position: Vector2<f32>);
    fn set_frame_info(&mut self, frame_count: u64, total_time: Duration);
    fn vision_grid(&'b mut self) -> Self::VisionCellGrid;
}

pub trait FrontendOutput<'a> {
    type WorldState: for<'b> OutputWorldState<'a, 'b>;
    fn with_world_state<F: FnMut(&mut Self::WorldState)>(&'a mut self, f: F);
    fn draw(&mut self);
    fn handle_resize(&mut self, width: u16, height: u16);
    fn update_world_size(&mut self, width: u32, height: u32);
}

pub trait FrontendInput {
    fn with_input<F: FnMut(Input)>(&mut self, f: F);
}
