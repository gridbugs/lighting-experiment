use entity_store::*;

#[test]
fn commit_into_store() {

    let mut es0 = EntityStore::new();
    let mut es1 = EntityStore::new();

    let mut change = EntityStoreChange::new();

    let e0 = 0;

    change.solid.insert(e0);
    es0.commit_change(&mut change);

    assert!(es0.solid.contains(&e0));
    assert!(!es1.solid.contains(&e0));

    change.solid.remove(e0);
    es0.commit_change_into_store(&mut change, &mut es1);
    assert!(!es0.solid.contains(&e0));
    assert!(es1.solid.contains(&e0));
}

#[test]
fn migration_copy() {
    let mut dest = EntityStore::new();
    let mut source = EntityStore::new();
    let mut dest_change = EntityStoreChange::new();
    let e0 = 0;
    let e1 = 1;

    source.solid.insert(e0);
    source.opacity.insert(e0, 0.2);
    dest.solid.insert(e1);
    dest.opacity.insert(e1, 0.4);

    migrate_data_copy!(source, dest, dest_change, e0, opacity);
    migrate_flag_copy!(source, dest, dest_change, e0, solid);
    migrate_data_copy!(source, dest, dest_change, e1, opacity);
    migrate_flag_copy!(source, dest, dest_change, e1, solid);

    dest.commit_change(&mut dest_change);

    assert_eq!(dest.opacity.get(&e0), Some(&0.2));
    assert_eq!(dest.opacity.get(&e1), None);
    assert!(dest.solid.contains(&e0));
    assert!(!dest.solid.contains(&e1));
}

#[test]
fn migration_move() {
    let mut dest = EntityStore::new();
    let mut source = EntityStore::new();
    let mut dest_change = EntityStoreChange::new();
    let mut source_change = EntityStoreChange::new();
    let e0 = 0;
    let e1 = 1;

    source.solid.insert(e0);
    source.opacity.insert(e0, 0.2);
    dest.solid.insert(e1);
    dest.opacity.insert(e1, 0.4);

    migrate_data_move!(source, dest, source_change, dest_change, e0, opacity);
    migrate_flag_move!(source, dest, source_change, dest_change, e0, solid);
    migrate_data_move!(source, dest, source_change, dest_change, e1, opacity);
    migrate_flag_move!(source, dest, source_change, dest_change, e1, solid);

    dest.commit_change(&mut dest_change);
    source.commit_change(&mut source_change);

    assert_eq!(dest.opacity.get(&e0), Some(&0.2));
    assert_eq!(dest.opacity.get(&e1), None);
    assert!(dest.solid.contains(&e0));
    assert!(!dest.solid.contains(&e1));
    assert_eq!(source.opacity.get(&e0), None);
    assert!(!source.solid.contains(&e0));
}

#[test]
fn migration_swap() {
    let mut dest = EntityStore::new();
    let mut source = EntityStore::new();
    let mut dest_change = EntityStoreChange::new();
    let mut source_change = EntityStoreChange::new();
    let e0 = 0;
    let e1 = 1;

    source.solid.insert(e0);
    source.opacity.insert(e0, 0.2);
    dest.opacity.insert(e0, 0.6);
    dest.solid.insert(e1);
    dest.opacity.insert(e1, 0.4);

    migrate_data_swap!(source, dest, source_change, dest_change, e0, opacity);
    migrate_flag_swap!(source, dest, source_change, dest_change, e0, solid);
    migrate_data_swap!(source, dest, source_change, dest_change, e1, opacity);
    migrate_flag_swap!(source, dest, source_change, dest_change, e1, solid);

    dest.commit_change(&mut dest_change);
    source.commit_change(&mut source_change);

    assert_eq!(dest.opacity.get(&e0), Some(&0.2));
    assert_eq!(dest.opacity.get(&e1), None);
    assert!(dest.solid.contains(&e0));
    assert!(!dest.solid.contains(&e1));
    assert_eq!(source.opacity.get(&e0), Some(&0.6));
    assert_eq!(source.opacity.get(&e1), Some(&0.4));
    assert!(!source.solid.contains(&e0));
}

#[test]
fn component_type_set() {

    let mut set = ComponentTypeSet::new();

    assert!(set.is_empty());
    assert!(!set.contains(ComponentType::Position));
    assert!(!set.contains(ComponentType::Solid));

    set.insert(ComponentType::Position);

    assert!(!set.is_empty());
    assert!(set.contains(ComponentType::Position));
    assert!(!set.contains(ComponentType::Solid));

    set.insert(ComponentType::Solid);
    assert!(!set.is_empty());
    assert!(set.contains(ComponentType::Position));
    assert!(set.contains(ComponentType::Solid));

    set.remove(ComponentType::Position);
    assert!(!set.is_empty());
    assert!(!set.contains(ComponentType::Position));
    assert!(set.contains(ComponentType::Solid));

    set.remove(ComponentType::Solid);
    assert!(set.is_empty());
    assert!(!set.contains(ComponentType::Position));
    assert!(!set.contains(ComponentType::Solid));

    let mut iter = set.iter();
    assert_eq!(iter.next(), None);

    set.insert(ComponentType::Position);
    set.insert(ComponentType::Solid);

    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(ComponentType::Position));
    assert_eq!(iter.next(), Some(ComponentType::Solid));
    assert_eq!(iter.next(), None);
}
