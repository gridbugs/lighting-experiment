#![allow(dead_code)]
#![allow(unused_macros)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;

mod static_grid;
mod limits;
mod direction;

#[macro_use] mod entity_store;
mod spatial_hash;

mod content;

fn main() {
    println!("Hello, world!");
}
