#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    Player,
    Npc,
}

pub const NUM_TURN_STATES: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnInfo {
    pub state: TurnState,
    pub count: u64,
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

impl TurnInfo {
    pub fn next(self) -> Self {
        Self {
            state: self.state.next_state(),
            count: self.count + 1,
        }
    }
}
