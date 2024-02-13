//! Split code into a library and binary for organisational purposes.

mod camera;
mod canvas;
mod colour;
mod intersection;
mod light;
mod material;
pub mod math;
mod object;
mod output;
mod pattern;
mod scene;
mod world;

pub use camera::Camera;
pub use canvas::Canvas;
pub use colour::Colour;
pub use light::Light;
pub use material::Material;
pub use object::{Object, Operation};
pub use output::Output;
pub use pattern::Pattern;
pub use world::World;
