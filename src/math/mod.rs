//! The math module provides mathematical primitives and functions that are
//! needed throughout the ray tracer.

mod angle;
pub mod float;
mod matrix;
mod point;
mod ray;
mod transformation;
mod vector;

pub use angle::Angle;
use matrix::Matrix;
pub use point::Point;
pub use ray::Ray;
pub use transformation::{Transformable, Transformation};
pub use vector::Vector;
