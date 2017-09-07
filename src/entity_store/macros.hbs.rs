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
    {{#if store}}
        {{#if type}}
            pub {{@key}}: {{store}}<{{type}}>,
        {{else}}
            pub {{@key}}: {{store}},
        {{/if}}
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! entity_store_cons {
    ($EntityStore:ident) => {
        $EntityStore {
{{#each components}}
    {{#if store}}
        {{#if type}}
                {{@key}}: {{store}}::default(),
        {{else}}
                {{@key}}: {{store}}::default(),
        {{/if}}
    {{/if}}
{{/each}}
        }
    }
}

macro_rules! commit {
    ($self:ident, $entity_change:ident) => {
        {
            match $entity_change {
                EntityChange::Insert(id, value) => {
                    match value {
{{#each components}}
    {{#if store}}
        {{#if type}}
                        ComponentValue::{{name}}(value) => { $self.{{@key}}.insert(id, value); }
        {{else}}
                        ComponentValue::{{name}} => { $self.{{@key}}.insert(id); }
        {{/if}}
    {{else}}
        {{#if type}}
                        ComponentValue::{{name}}(_) => {}
        {{else}}
                        ComponentValue::{{name}} => {}
        {{/if}}
    {{/if}}
{{/each}}
                    }
                }
                EntityChange::Remove(id, typ) => {
                    match typ {
{{#each components}}
    {{#if store}}
                        ComponentType::{{name}} => { $self.{{@key}}.remove(&id); },
    {{else}}
                        ComponentType::{{name}} => {},
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
            use entity_store::{EntityId, EntityChange, ComponentValue};
            entity_store_imports!{}

{{#each components}}
    {{#if type}}
            pub fn {{@key}}(id: EntityId, value: {{type}}) -> EntityChange {
                EntityChange::Insert(id, ComponentValue::{{name}}(value))
            }
    {{else}}
            pub fn {{@key}}(id: EntityId) -> EntityChange {
                EntityChange::Insert(id, ComponentValue::{{name}})
            }
    {{/if}}
{{/each}}
        }
    }
}


macro_rules! remove_shorthands {
    ($remove:ident) => {
        pub mod $remove {
            use entity_store::{EntityId, EntityChange, ComponentType};

{{#each components}}
            pub fn {{@key}}(id: EntityId) -> EntityChange {
                EntityChange::Remove(id, ComponentType::{{name}})
            }
{{/each}}
        }
    }
}
