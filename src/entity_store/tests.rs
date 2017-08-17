use entity_store::*;

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

#[test]
fn entity_vec_map() {
    let mut map = EntityVecMap::new();

    map.insert(0, 0);
    map.insert(128, 10);
    map.insert(2, 20);

    assert!(map.contains_key(&0));
    assert_eq!(map.get(&128), Some(&10));

    assert_eq!(map.remove(&2), Some(20));
    assert_eq!(map.remove(&2), None);
    assert!(!map.contains_key(&2));

    let mut iter = map.iter();
    assert_eq!(iter.next(), Some((0, &0)));
    assert_eq!(iter.next(), Some((128, &10)));
    assert_eq!(iter.next(), None);
}

#[test]
fn entity_vec_set() {

    let mut set = EntityVecSet::new();

    assert!(set.is_empty());

    set.insert(128);
    set.insert(0);
    set.insert(1);
    set.insert(2);

    assert!(set.contains(&2));
    assert!(set.contains(&128));

    assert!(set.remove(&1));
    assert!(!set.remove(&1));

    let mut iter = set.iter();

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(128));
}
