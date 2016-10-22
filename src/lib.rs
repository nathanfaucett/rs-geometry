#![feature(collections)]
#![no_std]


extern crate collections;

extern crate hash_map;
extern crate insert;
extern crate shared;


mod attribute;
mod geometry;


pub use attribute::{Attribute, AttributeValue};
pub use geometry::Geometry;
