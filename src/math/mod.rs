//! The math module provides mathematical primitives and functions that are
//! needed throughout the ray tracer.

pub mod float;
mod matrix;
mod point;
pub mod ray;
mod transformation;
mod vector;

pub use point::Point;
pub use transformation::{Transformable, Transformation};
pub use vector::Vector;
