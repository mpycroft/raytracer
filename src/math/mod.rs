//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

#[macro_use]
pub mod approx;

mod matrix;
mod point;
mod ray;
mod transform;
mod vector;

use matrix::Matrix;
pub use point::Point;
pub use ray::Ray;
pub use transform::{Transform, Transformable};
pub use vector::Vector;
