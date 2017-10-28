extern crate entity_store_code_gen;

use entity_store_code_gen::GeneratedCode;

fn main() {
    GeneratedCode::generate(include_str!("spec.toml"))
        .expect("Failed to generate code")
        .save()
        .expect("Failed to save code");
}
