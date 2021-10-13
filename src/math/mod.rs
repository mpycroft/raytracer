//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

#[macro_use]
mod float;

mod point;
mod vector;

pub use point::Point;
pub use vector::Vector;
