use cgmath::Vector2;

use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;

use input::Input;

pub trait OutputWorldState<'a> {
    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable);
    fn set_player_position(&mut self, player_position: Vector2<f32>);
    fn set_frame_info(&mut self, time: u64);
}

pub trait FrontendOutput<'a> {
    type WorldState: OutputWorldState<'a>;
    fn with_world_state<F: FnMut(&mut Self::WorldState)>(&'a mut self, f: F);
    fn draw(&mut self);
    fn handle_resize(&mut self, width: u16, height: u16);
    fn update_world_size(&mut self, width: u32, height: u32);
}

pub trait FrontendInput {
    fn with_input<F: FnMut(Input)>(&mut self, f: F);
}

pub struct Frontend<Input: FrontendInput, Output: for<'a> FrontendOutput<'a>> {
    pub input: Input,
    pub output: Output,
}
