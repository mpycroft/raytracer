//! The main ray tracer code is split into a library containing nearly all the
//! code and the main binary. This provides a nice separation as well as making
//! it easier later on to do benchmarking and doc tests that have issues with
//! being in a binary.

#[macro_use]
pub mod util;
pub mod math;

mod camera;
mod canvas;
mod colour;
mod intersect;
mod material;
mod object;
mod pattern;
mod point_light;
mod world;

pub use camera::Camera;
pub use canvas::Canvas;
pub use colour::Colour;
pub use intersect::Intersectable;
pub use material::Material;
pub use object::Object;
pub use pattern::Pattern;
pub use point_light::PointLight;
pub use world::World;
