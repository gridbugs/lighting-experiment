// Generated file. Do not edit!

macro_rules! entity_store_imports {
    () => {
{{#each imports}}
        use {{this}};
{{/each}}
    }
}

macro_rules! entity_store_decl {
    ($EntityStore:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $EntityStore {
{{#each components}}
    {{#if type}}
            pub {{@key}}: EntityMap<{{type}}>,
    {{else}}
            pub {{@key}}: EntitySet,
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! entity_store_cons {
    ($EntityStore:ident) => {
        $EntityStore {
{{#each components}}
    {{#if type}}
            {{@key}}: EntityMap::default(),
    {{else}}
            {{@key}}: EntitySet::default(),
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! remove_entity {
    ($self:expr, $entity:expr, $store:expr) => {
        {
{{#each components}}
    {{#if type}}
            if $store.{{@key}}.contains_key(&$entity) {
                $self.{{@key}}.remove($entity);
            }
    {{else}}
            if $store.{{@key}}.contains(&$entity) {
                $self.{{@key}}.remove($entity);
            }
    {{/if}}
{{/each}}
        }

    }
}

macro_rules! entity_store_change_decl {
    ($EntityStoreChange:ident) => {
        #[derive(Debug, Clone)]
        pub struct $EntityStoreChange {
{{#each components}}
    {{#if type}}
            pub {{@key}}: DataComponentChange<{{type}}>,
    {{else}}
            pub {{@key}}: FlagComponentChange,
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! entity_store_change_cons {
    ($EntityStoreChange:ident) => {
        $EntityStoreChange {
{{#each components}}
    {{#if type}}
            {{@key}}: DataComponentChange(ChangeEntityMap::default()),
    {{else}}
            {{@key}}: FlagComponentChange(ChangeEntityMap::default()),
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! commit_change {
    ($self:ident, $source:ident) => {
        {
{{#each components}}
    {{#if type}}
    for (id, change) in $source.{{@key}}.0.drain() {
        match change {
            DataChangeType::Insert(v) => { $self.{{@key}}.insert(id, v); }
            DataChangeType::Remove => { $self.{{@key}}.remove(&id); }
        }
    }
    {{else}}
    for (id, change) in $source.{{@key}}.0.drain() {
        match change {
            FlagChangeType::Insert => { $self.{{@key}}.insert(id); }
            FlagChangeType::Remove => { $self.{{@key}}.remove(&id); }
        }
    }
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! commit_change_into {
    ($self:ident, $source:ident, $dest:ident) => {
        {
{{#each components}}
    {{#if type}}
    for (id, change) in $source.{{@key}}.0.drain() {
        if let Some(existing) = match change {
            DataChangeType::Insert(v) => $self.{{@key}}.insert(id, v),
            DataChangeType::Remove => $self.{{@key}}.remove(&id),
        } {
            $dest.{{@key}}.insert(id, existing);
        }
    }
    {{else}}
    for (id, change) in $source.{{@key}}.0.drain() {
        if match change {
            FlagChangeType::Insert => $self.{{@key}}.insert(id),
            FlagChangeType::Remove => $self.{{@key}}.remove(&id),
        } {
            $dest.{{@key}}.insert(id);
        }
    }
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! entity_store_change_clear {
    ($self:expr) => {
{{#each components}}
        $self.{{@key}}.0.clear();
{{/each}}
    }
}

macro_rules! enum_component_type {
    ($name:ident) => {
        enum_from_primitive! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
{{#each components}}
            {{name}} = {{index}},
{{/each}}
        }
        }
    }
}

macro_rules! enum_component_value {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub enum $name {
{{#each components}}
    {{#if type}}
            {{name}}({{type}}),
    {{else}}
            {{name}},
    {{/if}}
{{/each}}
        }
    }
}
