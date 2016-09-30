#![feature(collections)]
#![no_std]


extern crate collections;

extern crate shared;


mod attribute;
mod bone;
mod geometry;


pub use attribute::{Attribute, AttributeValue};
pub use bone::Bone;
pub use geometry::Geometry;
