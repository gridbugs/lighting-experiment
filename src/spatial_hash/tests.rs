use cgmath::Vector2;

use entity_store::*;
use spatial_hash::*;

use direction::Direction;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

struct Env {
    entity_store: EntityStore,
    spatial_hash: SpatialHashTable,
    time: u64,
}

impl Env {
    fn new() -> Self {
        Env {
            entity_store: EntityStore::new(),
            spatial_hash: SpatialHashTable::new(WIDTH, HEIGHT),
            time: 0,
        }
    }

    fn commit(&mut self, change: EntityChange) {
        self.spatial_hash.update(&self.entity_store, &change, self.time);
        self.entity_store.commit(change);
        self.time += 1;
    }
}

#[test]
fn insert_change() {
    let mut env = Env::new();

    env.commit(insert::position(0, Vector2::new(0, 0).cast()));
    env.commit(insert::opacity(0, 0.5));

    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 5.0);

    env.commit(insert::opacity(0, 0.8));

    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 8.0);
}

#[test]
fn insert_move() {
    let mut env = Env::new();

    env.commit(insert::position(0, Vector2::new(0, 0).cast()));
    env.commit(insert::opacity(0, 0.5));

    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 5.0);

    env.commit(insert::position(0, Vector2::new(1, 0).cast()));

    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 0.0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 0)).unwrap().opacity_total * 10.0).round(), 5.0);
}

#[test]
fn redundant() {
    let mut env = Env::new();

    env.commit(insert::position(0, Vector2::new(0, 0).cast()));
    env.commit(insert::solid(0));
    env.commit(insert::solid(0));

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);

    env.commit(remove::solid(0));
    env.commit(remove::solid(0));

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 0);
}

#[test]
fn neighbour_count_insert() {
    let mut env = Env::new();

    env.commit(insert::position(0, Vector2::new(1, 1).cast()));
    env.commit(insert::wall(0));

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().wall_neighbours.get(Direction::SouthEast), 1);
}
