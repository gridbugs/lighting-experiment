#![allow(dead_code)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate handlebars;

#[path = "src/res/files.rs"]
mod files;

#[path = "src/simple_file.rs"]
mod simple_file;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{BTreeMap, BTreeSet};

use handlebars::Handlebars;

// files in the resources dir
const COMPONENT_SPEC: &'static str = "components.toml";
const SPATIAL_HASH_SPEC: &'static str = "spatial_hash.toml";

const ENTITY_STORE_MACROS: &'static str = "src/entity_store/macros.gen.rs";
const ENTITY_STORE_MACROS_TEMPLATE: &'static str = "src/entity_store/macros.hbs.rs";
const ENTITY_STORE_CONSTANTS: &'static str = "src/entity_store/constants.gen.rs";
const ENTITY_STORE_CONSTANTS_TEMPLATE: &'static str = "src/entity_store/constants.hbs.rs";

const SPATIAL_HASH_MACROS: &'static str = "src/spatial_hash/macros.gen.rs";
const SPATIAL_HASH_TEMPLATE: &'static str = "src/spatial_hash/macros.hbs.rs";

const RES_SRC_DIR: &'static str = "src/res";

const WORD_SIZE: usize = 64;

fn manifest_dir() -> PathBuf {
    PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"))
}

fn res_src_dir() -> PathBuf {
    manifest_dir().join(RES_SRC_DIR)
}

fn res_src_path<P: AsRef<Path>>(path: P) -> PathBuf {
    res_src_dir().join(path)
}

fn ret_none() -> Option<String> { None }

fn dst_dirs() -> Vec<PathBuf> {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let profile = env::var("PROFILE").unwrap();

    if target == host {
        vec![
           Path::new("target").join(&profile),
           Path::new("target").join(&target).join(&profile),
        ]
    } else {
        vec![Path::new("target").join(&target).join(&profile)]
    }.iter().filter(|p| p.exists()).cloned().collect()
}

#[derive(Debug, Deserialize)]
struct SpatialHashDesc {
    imports: BTreeSet<String>,
    position_component: String,
    fields: BTreeMap<String, SpatialHashFieldDesc>,
}

#[derive(Debug, Deserialize)]
struct SpatialHashFieldDesc {
    component: String,
    aggregate: String,
}

#[derive(Debug, Serialize)]
struct SpatialHashDescOut {
    imports: BTreeSet<String>,
    position_component: String,
    position_type: String,
    components: BTreeMap<String, SpatialHashComponentDescOut>,
}

#[derive(Debug, Serialize)]
struct SpatialHashComponentDescOut {
    #[serde(rename = "type", default = "ret_none")]
    type_name: Option<String>,
    fields: BTreeMap<String, SpatialHashFieldDescOut>,
}

#[derive(Debug, Serialize)]
struct SpatialHashFieldDescOut {
    aggregate_name: String,
    aggregate_type: String,
    aggregate_cons: String,
    void: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ComponentDesc {
    #[serde(rename = "type", default = "ret_none")]
    type_name: Option<String>,
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct ComponentDescOut {
    #[serde(rename = "type")]
    type_name: Option<String>,
    name: String,
    uppercase_name: String,
    index: usize,
    word_index: usize,
    word_bitmask: u64,
}

#[derive(Debug, Deserialize)]
struct EntityStoreDesc {
    imports: Vec<String>,
    components: BTreeMap<String, ComponentDesc>,
}

#[derive(Debug, Serialize)]
struct EntityStoreDescOut {
    imports: Vec<String>,
    components: BTreeMap<String, ComponentDescOut>,
    num_component_types: usize,
    num_component_type_words: usize,
    word_size: usize,
}

fn read_entity_store_desc<P: AsRef<Path>>(path: P) -> EntityStoreDesc {
    simple_file::read_toml(path).expect("Failed to parse entity store desc")
}

fn read_spatial_hash_desc<P: AsRef<Path>>(path: P) -> SpatialHashDesc {
    simple_file::read_toml(path).expect("Failed to parse spatial hash desc")
}

fn make_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();
    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars
}

fn render_entity_system_template_internal<P: AsRef<Path>>(desc: &EntityStoreDesc,
                                   template_path: P) -> String {

    let template = simple_file::read_string(template_path).expect("Failed to read template");

    let mut components = BTreeMap::new();

    for (index, (field, desc)) in desc.components.iter().enumerate() {
        let desc_out = ComponentDescOut {
            type_name: desc.type_name.clone(),
            name: desc.name.clone(),
            uppercase_name: desc.name.to_uppercase(),
            index,
            word_index: index / WORD_SIZE,
            word_bitmask: 1 << (index % WORD_SIZE),
        };
        components.insert(field.clone(), desc_out);
    }

    let entity_store_desc_out = EntityStoreDescOut {
        imports: desc.imports.clone(),
        num_component_types: desc.components.len(),
        num_component_type_words: ((desc.components.len() - 1) / WORD_SIZE) + 1,
        components,
        word_size: WORD_SIZE,
    };

    make_handlebars().template_render(template.as_ref(), &entity_store_desc_out)
        .expect("Failed to render template")
}

fn render_spatial_hash_template_internal<P: AsRef<Path>>(desc: SpatialHashDesc,
                                                         type_desc: EntityStoreDesc,
                                                         template_path: P) -> String {

    let template = simple_file::read_string(template_path).expect("Failed to read template");

    let SpatialHashDesc { imports, position_component, fields } = desc;
    let EntityStoreDesc { components, .. } = type_desc;

    let mut components_out = BTreeMap::new();

    for (field_name, field) in fields.iter() {
        let component_desc = components.get(&field.component).expect(&format!("No such component: {}", field_name));

        let (aggregate_type, aggregate_cons) = match field.aggregate.as_ref() {
            "count" => ("usize", "0"),
            "f64_total" => ("f64", "0.0"),
            "set" => ("HashSet<EntityId>", "HashSet::new()"),
            "neighbour_count" => ("NeighbourCount", "NeighbourCount::new()"),
            "void" => ("", ""),
            other => panic!("No such aggregate: {}", other),
        };

        let mut component = components_out.entry(field.component.clone()).or_insert_with(|| SpatialHashComponentDescOut {
            type_name: component_desc.type_name.clone(),
            fields: BTreeMap::new(),
        });

        let field_out = SpatialHashFieldDescOut {
            aggregate_name: field_name.clone(),
            aggregate_type: aggregate_type.to_string(),
            aggregate_cons: aggregate_cons.to_string(),
            void: field.aggregate == "void",
        };

        component.fields.insert(field.aggregate.clone(), field_out);
    }

    let desc_out = SpatialHashDescOut {
        imports: imports,
        position_component: position_component.clone(),
        position_type: components.get(&position_component)
            .expect(&format!("No such component: {}", &position_component))
            .type_name.clone().expect("Position component must have associated data"),
        components: components_out,
    };

    make_handlebars().template_render(template.as_ref(), &desc_out)
        .expect("Failed to render template")
}


fn render_entity_system_template() {

    let in_path = &res_src_path(COMPONENT_SPEC);
    let macros_out_path = ENTITY_STORE_MACROS;
    let constants_out_path = ENTITY_STORE_CONSTANTS;

    let macros_template_path = ENTITY_STORE_MACROS_TEMPLATE;
    let constants_template_path = ENTITY_STORE_CONSTANTS_TEMPLATE;

    if source_changed_rel(in_path, macros_out_path) ||
        source_changed_rel(in_path, constants_out_path) ||
        source_changed_rel(macros_template_path, macros_out_path) ||
        source_changed_rel(macros_template_path, constants_out_path)
    {
        let type_desc = read_entity_store_desc(in_path);
        let macros_output = render_entity_system_template_internal(&type_desc, macros_template_path);
        let constants_output = render_entity_system_template_internal(&type_desc, constants_template_path);
        simple_file::write_string(macros_out_path, macros_output).expect("Failed to write entity system macros");
        simple_file::write_string(constants_out_path, constants_output).expect("Failed to write entity system constants");
    }
}

fn render_spatial_hash_template() {
    let in_path = &res_src_path(SPATIAL_HASH_SPEC);
    let out_path = SPATIAL_HASH_MACROS;

    let template_path = SPATIAL_HASH_TEMPLATE;
    let component_spec = &res_src_path(COMPONENT_SPEC);

    if source_changed_rel(in_path, out_path) || source_changed_rel(template_path, out_path) {
        let desc = read_spatial_hash_desc(in_path);
        let type_desc = read_entity_store_desc(component_spec);
        let output = render_spatial_hash_template_internal(desc, type_desc, template_path);
        simple_file::write_string(out_path, output).expect("Failed to write spatial hash code");
    }
}

fn source_changed_rel<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) -> bool {
    if !out_path.as_ref().exists() {
        return true;
    }
    let out_time = if let Ok(md) = fs::metadata(out_path) {
        md.modified().expect("Failed to get output file modified time")
    } else {
        return true;
    };

    let in_time = fs::metadata(in_path).expect("Missing input file")
        .modified().expect("Failed to get input file modified time");

    in_time > out_time
}

fn ensure_dir<P: AsRef<Path>>(path: P) {
    if !path.as_ref().exists() {
        fs::create_dir(path).expect("Failed to create dir");
    }
}

fn copy_sprite_sheet() {
    let in_path = &res_src_path(files::SPRITE_SHEET);

    for dest in dst_dirs().iter() {
        let out_dir = dest.join(files::RES_DIR);
        ensure_dir(&out_dir);

        let out_path = out_dir.join(files::SPRITE_SHEET);

        if source_changed_rel(in_path, &out_path) {
            fs::copy(in_path, &out_path)
                .expect("Failed to copy sprite sheet");
        }
    }
}

fn main() {
    render_entity_system_template();
    render_spatial_hash_template();
    copy_sprite_sheet();
}
