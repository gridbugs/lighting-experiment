use direction::CardinalDirection;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ActionControl {
    Move(CardinalDirection),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MetaControl {
    Menu,
    Quit,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameControl {
    Action(ActionControl),
    Meta(MetaControl),
}
