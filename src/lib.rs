//! Split code into a library and binary for organisational purposes.

mod canvas;
mod colour;
pub mod intersect;
pub mod math;
mod sphere;

pub use canvas::Canvas;
pub use colour::Colour;
pub use sphere::Sphere;
