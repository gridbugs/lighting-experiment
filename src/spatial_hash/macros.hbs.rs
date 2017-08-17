// Generated file. Do not edit!

macro_rules! spatial_hash_imports {
    () => {
{{#each imports}}
        use {{this}};
{{/each}}
    }
}

macro_rules! spatial_hash_cell_decl {
    ($SpatialHashCell:ident) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $SpatialHashCell {
{{#each components}}
    {{#each fields}}
        {{#unless void}}
            pub {{aggregate_name}}: {{aggregate_type}},
        {{/unless}}
    {{/each}}
{{/each}}
            pub entities: fnv::FnvHashSet<EntityId>,
            pub last_updated: u64,

        }
    }
}

macro_rules! spatial_hash_cell_cons {
    ($SpatialHashCell:ident) => {
        $SpatialHashCell {
{{#each components}}
    {{#each fields}}
        {{#unless void}}
            {{aggregate_name}}: {{aggregate_cons}},
        {{/unless}}
    {{/each}}
{{/each}}
            entities: fnv::FnvHashSet::default(),
            last_updated: 0,
        }
    }
}

macro_rules! insert {
    ($self:expr, $entity_id:expr, $store:expr) => {
{{#each components}}
    {{#if type}}
        {{#if fields.f64_total}}
        if let Some(v) = $store.{{@key}}.get(&$entity_id) {
            $self.{{fields.f64_total.aggregate_name}} += *v;
        }
        {{/if}}
        {{#if fields.count}}
        if $store.{{@key}}.contains_key(&$entity_id) {
            $self.{{fields.count.aggregate_name}} += 1;
        }
        {{/if}}
        {{#if fields.set}}
        if $store.{{@key}}.contains_key(&$entity_id) {
            $self.{{fields.set.aggregate_name}}.insert($entity_id);
        }
        {{/if}}
    {{else}}
        {{#if fields.count}}
        if $store.{{@key}}.contains(&$entity_id) {
            $self.{{fields.count.aggregate_name}} += 1;
        }
        {{/if}}
        {{#if fields.set}}
        if $store.{{@key}}.contains(&$entity_id) {
            $self.{{fields.set.aggregate_name}}.insert($entity_id);
        }
        {{/if}}
    {{/if}}
{{/each}}
    }
}

macro_rules! remove {
    ($self:expr, $entity_id:expr, $store:expr) => {
{{#each components}}
    {{#if type}}
        {{#if fields.f64_total}}
        if let Some(v) = $store.{{@key}}.get(&$entity_id) {
            $self.{{fields.f64_total.aggregate_name}} -= *v;
        }
        {{/if}}
        {{#if fields.count}}
        if $store.{{@key}}.contains_key(&$entity_id) {
            $self.{{fields.count.aggregate_name}} -= 1;
        }
        {{/if}}
        {{#if fields.set}}
        if $store.{{@key}}.contains_key(&$entity_id) {
            $self.{{fields.set.aggregate_name}}.remove(&$entity_id);
        }
        {{/if}}
    {{else}}
        {{#if fields.count}}
        if $store.{{@key}}.contains(&$entity_id) {
            $self.{{fields.count.aggregate_name}} -= 1;
        }
        {{/if}}
        {{#if fields.set}}
        if $store.{{@key}}.contains(&$entity_id) {
            $self.{{fields.set.aggregate_name}}.remove(&$entity_id);
        }
        {{/if}}
    {{/if}}
{{/each}}
    }
}

macro_rules! insert_neighbours {
    ($self:expr, $entity_id:expr, $store:expr, $coord:expr) => {
{{#each components}}
    {{#if fields.neighbour_count}}
        {{#if type}}
        if $store.{{@key}}.contains_key(&$entity_id) {
        {{else}}
        if $store.{{@key}}.contains(&$entity_id) {
        {{/if}}
            for d in Directions {
                if let Some(mut cell) = $self.grid.get_signed_mut($coord.cast() + d.vector()) {
                    cell.{{fields.neighbour_count.aggregate_name}}.inc(d.opposite());
                }
            }
        }
    {{/if}}
{{/each}}
    }
}

macro_rules! remove_neighbours {
    ($self:expr, $entity_id:expr, $store:expr, $coord:expr) => {
{{#each components}}
    {{#if fields.neighbour_count}}
        {{#if type}}
        if $store.{{@key}}.contains_key(&$entity_id) {
        {{else}}
        if $store.{{@key}}.contains(&$entity_id) {
        {{/if}}
            for d in Directions {
                if let Some(mut cell) = $self.grid.get_signed_mut($coord.cast() + d.vector()) {
                    cell.{{fields.neighbour_count.aggregate_name}}.dec(d.opposite());
                }
            }
        }
    {{/if}}
{{/each}}
    }
}

macro_rules! insert_match {
    ($self:expr, $store:expr, $id:expr, $component:expr, $time:expr) => {
        match $component {
            &ComponentValue::{{position_name}}(position) => {
                if let Some(current) = $store.{{position_component}}.get(&$id) {
                    if let Some(mut cell) = $self.grid.get_signed_mut(current.cast()) {
                        cell.remove($id, $store, $time);
                    }
                    remove_neighbours!($self, $id, $store, current);
                }
                if let Some(mut cell) = $self.grid.get_signed_mut(position.cast()) {
                    cell.insert($id, $store, $time);
                }
                insert_neighbours!($self, $id, $store, position);
            }
{{#each components}}
    {{#if type}}
            &ComponentValue::{{name}}(value) => {
                if let Some(position) = $store.{{../position_component}}.get(&$id) {
        {{#if fields.neighbour_count}}
                    if !$store.{{@key}}.contains_key(&$id) {
                        for d in Directions {
                            if let Some(mut cell) = $self.grid.get_signed_mut(position.cast() + d.vector()) {
                                cell.{{fields.neighbour_count.aggregate_name}}.inc(d.opposite());
                            }
                        }
                    }
        {{/if}}
                    if let Some(mut cell) = $self.grid.get_signed_mut(position.cast()) {
                        if let Some(old) = $store.{{@key}}.get(&$id) {
        {{#if fields.f64_total}}
                            let increase = value - *old;
                            cell.{{fields.f64_total.aggregate_name}} += increase;
                            cell.last_updated = $time;
        {{/if}}
                        } else {
        {{#if fields.f64_total}}
                            cell.{{fields.f64_total.aggregate_name}} += value;
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.count}}
                            cell.{{fields.count.aggregate_name}} += 1;
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                            cell.{{fields.set.aggregate_name}}.insert($id);
                            cell.last_updated = $time;
        {{/if}}
                        }
        {{#if fields.void}}
                        cell.last_updated = $time;
        {{/if}}
                    }


                }
            }
    {{else}}
            &ComponentValue::{{name}} => {
                if let Some(position) = $store.{{../position_component}}.get(&$id) {
        {{#if fields.neighbour_count}}
                    if !$store.{{@key}}.contains(&$id) {
                        for d in Directions {
                            if let Some(mut cell) = $self.grid.get_signed_mut(position.cast() + d.vector()) {
                                cell.{{fields.neighbour_count.aggregate_name}}.inc(d.opposite());
                            }
                        }
                    }
        {{/if}}
                    if let Some(mut cell) = $self.grid.get_signed_mut(position.cast()) {
                        if !$store.{{@key}}.contains(&$id) {
        {{#if fields.count}}
                            cell.{{fields.count.aggregate_name}} += 1;
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                            cell.{{fields.set.aggregate_name}}.insert($id);
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.void}}
                            cell.last_updated = $time;
        {{/if}}
                        }
                    }
                }
            }
    {{/if}}
{{/each}}
            _ => {}
        }
    }
}

macro_rules! remove_match {
    ($self:expr, $store:expr, $id:expr, $typ:expr, $time:expr) => {
        match $typ {
            ComponentType::{{position_name}} => {
                if let Some(current) = $store.{{position_component}}.get(&$id) {
                    if let Some(mut cell) = $self.grid.get_signed_mut(current.cast()) {
                        cell.remove($id, $store, $time);
                    }
                    remove_neighbours!($self, $id, $store, current);
                }
            }
{{#each components}}
            ComponentType::{{name}} => {
                if let Some(position) = $store.{{../position_component}}.get(&$id) {
    {{#if type}}
        {{#if fields.neighbour_count}}
                    if $store.{{@key}}.contains_key(entity_id) {
                        for d in Directions {
                            if let Some(mut cell) = $self.grid.get_signed_mut(position.cast() + d.vector()) {
                                cell.{{fields.neighbour_count.aggregate_name}}.dec(d.opposite());
                            }
                        }
                    }
        {{/if}}
                    if let Some(mut cell) = $self.grid.get_signed_mut(position.cast()) {
        {{#if fields.f64_total}}
                        if let Some(value) = $store.{{@key}}.get(&$id) {
                            cell.{{fields.f64_total.aggregate_name}} -= *value;
                            cell.last_updated = $time;
                        }
        {{/if}}
                        if $store.{{@key}}.contains_key(&$id) {
        {{#if fields.count}}
                            cell.{{fields.count.aggregate_name}} -= 1;
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                            cell.{{fields.set.aggregate_name}}.remove(&$id);
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.void}}
                            cell.last_updated = $time;
        {{/if}}
                        }
                    }
    {{else}}
        {{#if fields.neighbour_count}}
                    if $store.{{@key}}.contains(&$id) {
                        for d in Directions {
                            if let Some(mut cell) = $self.grid.get_signed_mut(position.cast() + d.vector()) {
                                cell.{{fields.neighbour_count.aggregate_name}}.dec(d.opposite());
                            }
                        }
                    }
        {{/if}}
                    if let Some(mut cell) = $self.grid.get_signed_mut(position.cast()) {
                        if $store.{{@key}}.contains(&$id) {
        {{#if fields.count}}
                            cell.{{fields.count.aggregate_name}} -= 1;
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                            cell.{{fields.set.aggregate_name}}.remove(&$id);
                            cell.last_updated = $time;
        {{/if}}
        {{#if fields.void}}
                            cell.last_updated = $time;
        {{/if}}
                        }
                    }
    {{/if}}
                }
            }
{{/each}}
            _ => {}
        }
    }
}
