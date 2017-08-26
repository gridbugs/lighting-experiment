use std::collections::HashMap;

use control::{GameControl, ActionControl, MetaControl};
use input::{Input, BindableInput, UnbindableInput};

pub struct GameControlTable {
    controls: HashMap<Input, GameControl>,
}

impl GameControlTable {
    pub fn new(mut bound_controls: HashMap<BindableInput, ActionControl>) -> Self {
        let mut controls = HashMap::new();

        let mut fixed_controls = hashmap!{
            UnbindableInput::Escape => MetaControl::Menu,
            UnbindableInput::Quit => MetaControl::Quit,
        };

        for (input, control) in bound_controls.drain() {
            controls.insert(Input::Bindable(input), GameControl::Action(control));
        }

        for (input, control) in fixed_controls.drain() {
            controls.insert(Input::Unbindable(input), GameControl::Meta(control));
        }

        Self {
            controls,
        }
    }

    pub fn get(&self, input: Input) -> Option<GameControl> {
        self.controls.get(&input).cloned()
    }
}
