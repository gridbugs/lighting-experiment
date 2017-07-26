#![allow(dead_code)]
#![allow(unused_macros)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;

extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

mod static_grid;
mod limits;
mod direction;

#[macro_use] mod entity_store;
mod spatial_hash;

mod content;

mod launch;
mod frontend;

fn main() {
    launch::launch(frontend::Frontend::new());
}
