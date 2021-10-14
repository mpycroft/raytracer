//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

#[macro_use]
pub mod float;

mod matrix;
mod point;
mod vector;

pub use point::Point;
pub use vector::Vector;
