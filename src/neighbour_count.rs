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

    pub fn bitmap(&self) -> u8 {
        (((self.neighbours[0] != 0) as u8) << 0) |
        (((self.neighbours[1] != 0) as u8) << 1) |
        (((self.neighbours[2] != 0) as u8) << 2) |
        (((self.neighbours[3] != 0) as u8) << 3) |
        (((self.neighbours[4] != 0) as u8) << 4) |
        (((self.neighbours[5] != 0) as u8) << 5) |
        (((self.neighbours[6] != 0) as u8) << 6) |
        (((self.neighbours[7] != 0) as u8) << 7)
    }
}
