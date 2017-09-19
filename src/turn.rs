#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    Player,
    Npc,
}

impl TurnState {
    pub fn next_state(self) -> Self {
        use self::TurnState::*;
        match self {
            Player => Npc,
            Npc => Player,
        }
    }
}
