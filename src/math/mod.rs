//! The math module provides mathematical primitives needed throughout the ray
//! tracer.

mod angle;
mod matrix;
mod perlin_noise;
mod point;
mod ray;
mod transform;
mod util;
mod vector;

pub use angle::Angle;
use matrix::Matrix;
pub use perlin_noise::PerlinNoise;
pub use point::Point;
pub use ray::Ray;
pub use transform::{Transform, Transformable};
pub use util::lerp;
pub use vector::Vector;
