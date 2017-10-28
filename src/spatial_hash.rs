use entity_store;
pub use entity_store::SpatialHashCell;
use cgmath::Vector2;
use static_grid::{self, StaticGrid};

pub struct SpatialHashTable {
    inner: entity_store::SpatialHashTable,
    dummy: StaticGrid<u8>,
}

impl SpatialHashTable {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: entity_store::SpatialHashTable::new(width, height),
            dummy: StaticGrid::new_default(width, height),
        }
    }

    pub fn width(&self) -> u32 { self.inner.width() }
    pub fn height(&self) -> u32 { self.inner.height() }

    pub fn get(&self, c: Vector2<u32>) -> Option<&SpatialHashCell> {
        self.inner.get(c)
    }
    pub fn get_signed(&self, c: Vector2<i32>) -> Option<&SpatialHashCell> {
        self.inner.get(c)
    }
    pub fn get_float(&self, c: Vector2<f32>) -> Option<&SpatialHashCell> {
        self.inner.get((c + Vector2::new(0.5, 0.5)).cast::<i32>())
    }
    pub fn update(&mut self, entity_store: &entity_store::EntityStore, change: &entity_store::EntityChange, time: u64) {
        self.inner.update(entity_store, change, time);
    }

    pub fn neighbour_coord_iter<IntoOffset, Iter, IntoIter>
        (&self, base: Vector2<u32>, into_iter: IntoIter) -> NeighbourCoordIter<IntoOffset, Iter>
    where IntoOffset: Into<Vector2<i32>>,
          Iter: Iterator<Item=IntoOffset>,
          IntoIter: IntoIterator<Item=IntoOffset, IntoIter=Iter>,
    {
        self.dummy.neighbour_coord_iter(base, into_iter)
    }
}

pub type NeighbourCoordIter<C, I> = static_grid::NeighbourCoordIter<C, I>;
