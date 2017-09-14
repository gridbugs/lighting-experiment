#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(inclusive_range_syntax)]

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

mod static_grid;
mod limits;
mod direction;
mod neighbour_count;
mod append;
mod vector_index;
mod util;

#[macro_use] mod entity_store;
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

fn main() {
    let (input, output) = glutin_frontend::create();
    launch::launch(input, output);
}
