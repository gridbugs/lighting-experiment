#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BindableInput {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Return,
    Space,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UnbindableInput {
    Escape,
    Quit,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Input {
    Bindable(BindableInput),
    Unbindable(UnbindableInput),
}
