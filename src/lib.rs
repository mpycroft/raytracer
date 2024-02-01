//! Split code into a library and binary for organisational purposes.

mod camera;
mod canvas;
mod colour;
mod intersection;
mod material;
pub mod math;
mod object;
mod output;
mod pattern;
mod point_light;
mod world;

pub use camera::Camera;
pub use canvas::Canvas;
pub use colour::Colour;
pub use material::Material;
pub use object::{Object, Operation};
pub use output::Output;
pub use pattern::Pattern;
pub use point_light::PointLight;
pub use world::World;
