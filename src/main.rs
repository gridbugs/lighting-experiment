#![allow(dead_code)]
#![allow(unused_macros)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate maplit;

#[macro_use] extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate genmesh;
extern crate image;

mod static_grid;
mod limits;
mod direction;
mod neighbour_count;

#[macro_use] mod entity_store;
mod spatial_hash;
mod entity_id_allocator;

mod depth;
mod prototype;
mod terrain;
mod content;
mod res;

mod launch;
mod frontend;
mod renderer;

fn main() {
    launch::launch(frontend::Frontend::new().expect("Failed to initialise frontend"));
}
