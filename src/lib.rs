//! Split code into a library and binary for organisational purposes.

mod bounding_box;
mod camera;
mod canvas;
mod colour;
mod intersection;
mod material;
pub mod math;
mod obj_parser;
mod object;
mod pattern;
mod point_light;
mod shape;
mod world;

pub use camera::Camera;
pub use canvas::Canvas;
pub use colour::Colour;
pub use material::Material;
pub use object::Object;
pub use pattern::Pattern;
pub use point_light::PointLight;
pub use world::World;
