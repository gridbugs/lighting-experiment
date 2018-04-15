#![allow(dead_code)]
#![allow(unused_macros)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate enum_primitive;
extern crate fnv;
extern crate num;
#[macro_use] extern crate itertools;
#[macro_use] extern crate maplit;
extern crate handlebars;
extern crate toml;

#[macro_use] extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate image;

extern crate direction;
#[macro_use] extern crate entity_store_helper;

mod static_grid;
mod limits;
mod neighbour_count;
mod append;
mod vector_index;
mod util;

mod entity_store { include_entity_store!("entity_store.rs"); }

mod spatial_hash;
mod id_allocator;
mod entity_id_allocator;
mod frame_env;

mod prototype;
mod terrain;
mod content;
mod res;

mod launch;
mod policy;
mod frontend;
mod glutin_frontend;
mod renderer;
mod input;
mod control;
mod control_table;
mod vision;
mod dijkstra_map;
mod search;
mod ai_info;
mod ai;
mod door_manager;
mod turn;
mod vec_pool;

fn main() {
    let (input, output) = glutin_frontend::create();
    launch::launch(input, output);
}
