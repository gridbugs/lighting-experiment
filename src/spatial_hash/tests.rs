use cgmath::Vector2;

use entity_store::*;
use spatial_hash::*;

use direction::Direction;

use content::DoorState;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

struct Env {
    entity_store: EntityStore,
    change: EntityStoreChange,
    spatial_hash: SpatialHashTable,
    time: u64,
}

impl Env {
    fn new() -> Self {
        Env {
            entity_store: EntityStore::new(),
            change: EntityStoreChange::new(),
            spatial_hash: SpatialHashTable::new(WIDTH, HEIGHT),
            time: 0,
        }
    }

    fn commit(&mut self) {
        self.spatial_hash.update(&self.entity_store, &self.change, self.time);
        self.entity_store.commit_change(&mut self.change);
        self.time += 1;
    }
}

#[test]
fn insert_change() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e0, 0.5);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 5.0);
    env.change.opacity.insert(e0, 0.8);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 8.0);
}

#[test]
fn insert_change_move_remove() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e0, 0.5);
    env.change.solid.insert(e0);
    env.change.door_state.insert(e0, DoorState::Open);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 5.0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().door_set.len(), 1);
    assert!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().door_set.contains(&e0));
    assert!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().entities.contains(&e0));

    env.change.opacity.insert(e0, 0.3);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 3.0);

    env.change.position.insert(e0, Vector2::new(1, 1).cast());
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 0.0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().door_set.len(), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().entities.len(), 0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 3.0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().solid_count, 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().door_set.len(), 1);
    assert!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().door_set.contains(&e0));
    assert!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().entities.contains(&e0));

    env.change.remove_entity(e0, &env.entity_store);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 0.0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().solid_count, 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().door_set.len(), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().entities.len(), 0);
}

#[test]
fn insert_and_move() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e0, 0.5);
    env.commit();

    env.change.position.insert(e0, Vector2::new(1, 1).cast());
    env.change.opacity.insert(e0, 0.3);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total, 0.0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 3.0);
}

#[test]
fn insert_and_remove() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e0, 0.5);
    env.change.solid.insert(e0);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 5.0);

    env.change.opacity.insert(e0, 0.3);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 3.0);

    env.change.solid.insert(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);

    env.change.position.remove(e0);
    env.change.opacity.insert(e0, 0.6);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 0);
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 0.0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().entities.len(), 0);
}

#[test]
fn change_and_move() {
    let mut env = Env::new();

    let e0 = 0;
    let e1 = 1;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e0, 0.5);
    env.change.position.insert(e1, Vector2::new(0, 0).cast());
    env.change.opacity.insert(e1, 0.3);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 8.0);

    env.change.position.insert(e0, Vector2::new(1, 1).cast());
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 3.0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 5.0);
}

#[test]
fn redundant_insert() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.solid.insert(e0);
    env.change.solid.insert(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);

    env.change.solid.insert(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);

    env.change.solid.insert(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);
}

#[test]
fn redundant_remove() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.solid.insert(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 1);

    env.change.solid.remove(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 0);

    env.change.solid.remove(e0);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().solid_count, 0);
}

#[test]
fn neighbour_count_insert() {
    let mut env = Env::new();

    let e0 = 0;
    let e1 = 1;
    let e2 = 2;
    let e3 = 3;

    env.change.position.insert(e0, Vector2::new(2, 2).cast());
    env.change.wall.insert(e0);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::SouthEast), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::West), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::East), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(2, 2)).unwrap().wall_neighbours.get(Direction::West), 0);

    env.change.position.insert(e1, Vector2::new(4, 2).cast());
    env.change.wall.insert(e1);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::West), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::East), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::SouthEast), 1);

    env.change.position.insert(e2, Vector2::new(4, 2).cast());
    env.change.wall.insert(e2);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::West), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(3, 2)).unwrap().wall_neighbours.get(Direction::East), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::SouthEast), 1);

    env.change.position.insert(e3, Vector2::new(0, 0).cast());
    env.change.wall.insert(e3);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 1);
}

#[test]
fn neighbour_count_insert_and_move() {
    let mut env = Env::new();

    let e0 = 0;
    let e1 = 1;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.position.insert(e1, Vector2::new(0, 0).cast());
    env.change.wall.insert(e0);
    env.change.wall.insert(e1);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::East), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().wall_neighbours.get(Direction::SouthEast), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::South), 0);

    env.change.position.insert(e1, Vector2::new(1, 1).cast());
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::East), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().wall_neighbours.get(Direction::SouthEast), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::South), 1);
}

#[test]
fn neighbour_count_insert_and_remove() {
    let mut env = Env::new();

    let mut env = Env::new();

    let e0 = 0;
    let e1 = 1;

    env.change.position.insert(e0, Vector2::new(0, 0).cast());
    env.change.position.insert(e1, Vector2::new(0, 0).cast());
    env.change.wall.insert(e0);
    env.change.wall.insert(e1);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 2);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 2);

    env.change.remove_entity(e0, &env.entity_store);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 1);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 1);

    env.change.position.remove(e1);
    env.commit();

    assert_eq!(env.spatial_hash.get(Vector2::new(0, 1)).unwrap().wall_neighbours.get(Direction::North), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 1)).unwrap().wall_neighbours.get(Direction::NorthWest), 0);
    assert_eq!(env.spatial_hash.get(Vector2::new(1, 0)).unwrap().wall_neighbours.get(Direction::West), 0);
}
