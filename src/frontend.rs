use cgmath::Vector2;

use entity_store::{EntityStore, EntityChange};
use spatial_hash::SpatialHashTable;

use input::InputEvent;

pub trait OutputWorldState<'a> {
    fn update(&mut self, change: &EntityChange, entity_store: &EntityStore, spatial_hash: &SpatialHashTable);
    fn set_player_position(&mut self, player_position: Vector2<f32>);
}

pub trait FrontendOutput<'a> {
    type WorldState: OutputWorldState<'a>;
    fn with_world_state<F: FnMut(&mut Self::WorldState)>(&'a mut self, f: F);
    fn draw(&mut self);
}

pub trait FrontendInput {
    fn with_input<F: FnMut(InputEvent)>(&mut self, f: F);
}

pub struct Frontend<Input: FrontendInput, Output: for<'a> FrontendOutput<'a>> {
    pub input: Input,
    pub output: Output,
}
