use direction::CardinalDirection;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Control {
    Move(CardinalDirection),
}
