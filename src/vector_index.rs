use cgmath::Vector2;
use direction::CardinalDirection;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VectorIndex {
    X,
    Y,
}

impl VectorIndex {
    pub fn get<T: Copy>(self, v: Vector2<T>) -> T {
        match self {
            VectorIndex::X => v.x,
            VectorIndex::Y => v.y,
        }
    }
    pub fn set<T>(self, v: &mut Vector2<T>, t: T) {
        match self {
            VectorIndex::X => v.x = t,
            VectorIndex::Y => v.y = t,
        }
    }
    pub fn create_coord(self, i: i32) -> Vector2<i32> {
        match self {
            VectorIndex::X => Vector2::new(i, 0),
            VectorIndex::Y => Vector2::new(0, i),
        }
    }
    pub fn get_tuple<T: Copy>(self, (x, y): (T, T)) -> T {
        match self {
            VectorIndex::X => x,
            VectorIndex::Y => y,
        }
    }
    pub fn from_card(direction: CardinalDirection) -> Self {
        match direction {
            CardinalDirection::North => VectorIndex::Y,
            CardinalDirection::East => VectorIndex::X,
            CardinalDirection::South => VectorIndex::Y,
            CardinalDirection::West => VectorIndex::X,
        }
    }
}
