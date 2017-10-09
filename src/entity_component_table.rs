use entity_store::{EntityVecMap, ComponentTypeSet, EntityChange, EntityId};
use append::Append;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentTable(EntityVecMap<ComponentTypeSet>);

impl EntityComponentTable {
    pub fn new() -> Self {
        EntityComponentTable(EntityVecMap::new())
    }
    pub fn update(&mut self, change: &EntityChange) {
        match change {
            &EntityChange::Insert(id, ref value) => {
                self.0.entry(&id).or_insert_with(|| ComponentTypeSet::new()).insert(value.typ());
            }
            &EntityChange::Remove(id, typ) => {
                if let Some(set) = self.0.get_mut(&id) {
                    set.remove(typ);
                }
            }
        }
    }
    pub fn delete_entity<A: Append<EntityChange>>(&self, id: EntityId, changes: &mut A) {
        if let Some(set) = self.0.get(&id) {
            for typ in set.iter() {
                changes.append(EntityChange::Remove(id, typ));
            }
        }
    }
}
