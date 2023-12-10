//! The math module provides mathematical primitives and functions that are
//! needed throughout the ray tracer.

pub mod float;
pub mod matrix;
mod point;
mod vector;

pub use point::Point;
pub use vector::Vector;
