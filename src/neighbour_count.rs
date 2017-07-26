use direction::Direction;

const NUM_NEIGHBOURS: usize = 8;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NeighbourCount {
    neighbours: [u8; NUM_NEIGHBOURS],
}

impl NeighbourCount {
    pub fn new() -> Self {
        NeighbourCount {
            neighbours: [0; NUM_NEIGHBOURS],
        }
    }

    pub fn inc(&mut self, direction: Direction) {
        self.neighbours[direction as usize] += 1;
    }

    pub fn dec(&mut self, direction: Direction) {
        self.neighbours[direction as usize] -= 1;
    }

    pub fn get(&self, direction: Direction) -> u8 {
        self.neighbours[direction as usize]
    }

    pub fn has(&self, direction: Direction) -> bool {
        self.neighbours[direction as usize] != 0
    }
}
