//! The math module provides mathematical primitives and functions that are
//! needed throughout the ray tracer.

pub mod float;
pub mod matrix;
mod point;
mod transformation;
mod vector;

pub use point::Point;
pub use transformation::Transformation;
pub use vector::Vector;
