use std::collections::HashMap;

use control::Control;
use input::Bindable;

pub struct GameControlTable {
    controls: HashMap<Bindable, Control>,
}

impl GameControlTable {
    pub fn new(controls: HashMap<Bindable, Control>) -> Self {
        Self {
            controls,
        }
    }

    pub fn get(&self, input: Bindable) -> Option<Control> {
        self.controls.get(&input).cloned()
    }
}
