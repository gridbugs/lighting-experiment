extern crate entity_store_code_gen;

fn main() {
    entity_store_code_gen::generate(include_str!("spec.toml")).unwrap()
}
