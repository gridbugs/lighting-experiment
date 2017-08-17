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

macro_rules! commit {
    ($self:ident, $entity_change:ident) => {
        {
            let EntityChange { id, change } = $entity_change;
            match change {
                Change::Insert(value) => {
                    match value {
{{#each components}}
    {{#if type}}
                        ComponentValue::{{name}}(value) => { $self.{{@key}}.insert(id, value); },
    {{else}}
                        ComponentValue::{{name}} => { $self.{{@key}}.insert(id); },
    {{/if}}
{{/each}}
                    }
                }
                Change::Remove(typ) => {
                    match typ {
{{#each components}}
    {{#if type}}
                        ComponentType::{{name}} => { $self.{{@key}}.remove(&id); },
    {{else}}
                        ComponentType::{{name}} => { $self.{{@key}}.remove(&id); },
    {{/if}}
{{/each}}
                    }
                }
            }
        }
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

macro_rules! insert_shorthands {
    ($insert:ident) => {
        pub mod $insert {
            use entity_store::{EntityId, EntityChange, Change, ComponentValue};
            entity_store_imports!{}

{{#each components}}
    {{#if type}}
            pub fn {{@key}}(id: EntityId, value: {{type}}) -> EntityChange {
                EntityChange {
                    id,
                    change: Change::Insert(ComponentValue::{{name}}(value)),
                }
            }
    {{else}}
            pub fn {{@key}}(id: EntityId) -> EntityChange {
                EntityChange {
                    id,
                    change: Change::Insert(ComponentValue::{{name}}),
                }
            }
    {{/if}}
{{/each}}
        }
    }
}


macro_rules! remove_shorthands {
    ($remove:ident) => {
        pub mod $remove {
            use entity_store::{EntityId, EntityChange, Change, ComponentType};

{{#each components}}
    {{#if type}}
            pub fn {{@key}}(id: EntityId) -> EntityChange {
                EntityChange {
                    id,
                    change: Change::Remove(ComponentType::{{name}}),
                }
            }
    {{else}}
            pub fn {{@key}}(id: EntityId) -> EntityChange {
                EntityChange {
                    id,
                    change: Change::Remove(ComponentType::{{name}}),
                }
            }
    {{/if}}
{{/each}}
        }
    }
}
