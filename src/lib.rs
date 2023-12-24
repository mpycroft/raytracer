//! Split code into a library and binary for organisational purposes.

mod camera;
mod canvas;
mod colour;
pub mod intersection;
mod material;
pub mod math;
mod object;
mod pattern;
mod point_light;
mod shape;
mod world;

pub use camera::Camera;
pub use canvas::Canvas;
pub use colour::Colour;
pub use intersection::Intersection;
pub use material::Material;
pub use object::Object;
pub use point_light::PointLight;
pub use shape::Shape;
pub use world::World;
