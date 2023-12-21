//! Split code into a library and binary for organisational purposes.

mod canvas;
mod colour;
pub mod intersect;
mod material;
pub mod math;
mod point_light;
mod sphere;

pub use canvas::Canvas;
pub use colour::Colour;
pub use material::Material;
pub use point_light::PointLight;
pub use sphere::Sphere;
