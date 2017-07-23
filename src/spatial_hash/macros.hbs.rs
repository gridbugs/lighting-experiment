// Generated file. Do not edit!

macro_rules! spatial_hash_imports {
    () => {
{{#each imports}}
        use {{this}};
{{/each}}
    }
}

macro_rules! position_type {
    () => {
        {{position_type}}
    }
}

macro_rules! position {
    ($store:expr) => {
        $store.{{position_component}}
    }
}

macro_rules! spatial_hash_cell_decl {
    ($SpatialHashCell:ident) => {
        #[derive(Serialize, Deserialize)]
        pub struct $SpatialHashCell {
{{#each components}}
    {{#each fields}}
        {{#unless void}}
            pub {{aggregate_name}}: {{aggregate_type}},
        {{/unless}}
    {{/each}}
{{/each}}
            pub entities: HashSet<EntityId>,
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
            entities: HashSet::new(),
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

macro_rules! update_component_loops {
    ($self:expr, $store:expr, $change:expr, $time:expr) => {
{{#each components}}
        for (entity_id, change) in $change.{{@key}}.iter() {
            match change {
    {{#if type}}
                &DataChangeType::Insert(v) => {
                    if let Some(position) = post_change_get!($store, $change, *entity_id, {{../position_component}}) {
                        if let Some(mut cell) = $self.grid.get_mut(position.into()) {
                            if let Some(old) = $store.{{@key}}.get(entity_id) {
        {{#if fields.f64_total}}
                                let increase = v - *old;
                                cell.{{fields.f64_total.aggregate_name}} += increase;
                                cell.last_updated = $time;
        {{/if}}
                            } else {
        {{#if fields.f64_total}}
                                cell.{{fields.f64_total.aggregate_name}} += v;
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.count}}
                                cell.{{fields.count.aggregate_name}} += 1;
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                                cell.{{fields.set.aggregate_name}}.insert(*entity_id);
                                cell.last_updated = $time;
        {{/if}}
                            }
        {{#if fields.void}}
                            cell.last_updated = $time;
        {{/if}}
                        }
                    }
                }
                &DataChangeType::Remove => {
                    if let Some(position) = post_change_get!($store, $change, *entity_id, {{../position_component}}) {
                        if let Some(mut cell) = $self.grid.get_mut(position.into()) {
        {{#if fields.f64_total}}
                            if let Some(value) = $store.{{@key}}.get(entity_id) {
                                cell.{{fields.f64_total.aggregate_name}} -= *value;
                                cell.last_updated = $time;
                            }
        {{/if}}
                            if $store.{{@key}}.contains_key(entity_id) {
        {{#if fields.count}}
                                cell.{{fields.count.aggregate_name}} -= 1;
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                                cell.{{fields.set.aggregate_name}}.remove(entity_id);
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.void}}
                                cell.last_updated = $time;
        {{/if}}
                            }
                        }
                    }
                }
    {{else}}
                &FlagChangeType::Insert => {
                    if let Some(position) = post_change_get!($store, $change, *entity_id, {{../position_component}}) {
                        if let Some(mut cell) = $self.grid.get_mut(position.into()) {
                            if !$store.{{@key}}.contains(entity_id) {
        {{#if fields.count}}
                                cell.{{fields.count.aggregate_name}} += 1;
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                                cell.{{fields.set.aggregate_name}}.insert(*entity_id);
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.void}}
                                cell.last_updated = $time;
        {{/if}}
                            }
                        }
                    }
                }
                &FlagChangeType::Remove => {
                    if let Some(position) = post_change_get!($store, $change, *entity_id, {{../position_component}}) {
                        if let Some(mut cell) = $self.grid.get_mut(position.into()) {
                            if $store.{{@key}}.contains(entity_id) {
        {{#if fields.count}}
                                cell.{{fields.count.aggregate_name}} -= 1;
                                cell.last_updated = $time;
        {{/if}}
        {{#if fields.set}}
                                cell.{{fields.set.aggregate_name}}.remove(entity_id);
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
            }
        }
{{/each}}
    }
}
