#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bindable {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Return,
    Space,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Unbindable {
    Escape,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum System {
    Quit,
    Resize(u16, u16),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Input {
    Bindable(Bindable),
    Unbindable(Unbindable),
    System(System),
}
