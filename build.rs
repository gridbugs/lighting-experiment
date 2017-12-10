#[macro_use] extern crate entity_store_code_gen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "src/res/files.rs"]
mod files;

const RES_SRC_DIR: &'static str = "src/res";

fn manifest_dir() -> PathBuf {
    PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"))
}

fn res_src_dir() -> PathBuf {
    manifest_dir().join(RES_SRC_DIR)
}

fn res_src_path<P: AsRef<Path>>(path: P) -> PathBuf {
    res_src_dir().join(path)
}

fn ensure_dir<P: AsRef<Path>>(path: P) {
    if !path.as_ref().exists() {
        fs::create_dir(path).expect("Failed to create dir");
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
    generate_entity_store!("spec.toml", "entity_store.rs");
    copy_sprite_sheet();
}
