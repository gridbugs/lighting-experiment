use cgmath::Vector2;

use entity_store::*;
use spatial_hash::*;

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

    env.change.position.insert(e0, Vector2::new(0, 0));
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

    env.change.position.insert(e0, Vector2::new(0, 0));
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

    env.change.position.insert(e0, Vector2::new(1, 1));
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

    env.change.position.insert(e0, Vector2::new(0, 0));
    env.change.opacity.insert(e0, 0.5);
    env.commit();

    env.change.position.insert(e0, Vector2::new(1, 1));
    env.change.opacity.insert(e0, 0.3);
    env.commit();
    assert_eq!(env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total, 0.0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 3.0);
}

#[test]
fn insert_and_remove() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0));
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

    env.change.position.insert(e0, Vector2::new(0, 0));
    env.change.opacity.insert(e0, 0.5);
    env.change.position.insert(e1, Vector2::new(0, 0));
    env.change.opacity.insert(e1, 0.3);
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 8.0);

    env.change.position.insert(e0, Vector2::new(1, 1));
    env.commit();
    assert_eq!((env.spatial_hash.get(Vector2::new(0, 0)).unwrap().opacity_total * 10.0).round(), 3.0);
    assert_eq!((env.spatial_hash.get(Vector2::new(1, 1)).unwrap().opacity_total * 10.0).round(), 5.0);
}

#[test]
fn redundant_insert() {
    let mut env = Env::new();

    let e0 = 0;

    env.change.position.insert(e0, Vector2::new(0, 0));
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

    env.change.position.insert(e0, Vector2::new(0, 0));
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
