#![feature(alloc)]
#![feature(collections)]
#![no_std]


extern crate alloc;
extern crate collections;

extern crate uuid;

extern crate hash_map;
extern crate insert;
extern crate shared;


mod attribute;
mod geometry;


pub use attribute::{Attribute, AttributeValue};
pub use geometry::Geometry;
