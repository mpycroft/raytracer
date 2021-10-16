//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

#[macro_use]
pub mod approx;

mod matrix;
mod point;
mod ray;
mod vector;

pub use matrix::Matrix;
pub use point::Point;
pub use vector::Vector;
