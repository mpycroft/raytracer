//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

#[macro_use]
pub mod approx;

mod angle;
// mod matrix;
mod point;
// mod ray;
// mod transform;
mod vector;

pub use angle::Angle;
// use matrix::Matrix;
pub use point::Point;
// pub use ray::Ray;
// pub use transform::{Transform, Transformable};
pub use vector::Vector;
