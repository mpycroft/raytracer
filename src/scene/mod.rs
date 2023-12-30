mod chapter10;
mod chapter6;
mod chapter8;
mod chapter9;

use clap::ValueEnum;
use derive_more::{Constructor, Display};
use raytracer::{Camera, Canvas, World};

/// `Scene` is a list of all the scenes we know about.
#[derive(Clone, Copy, Debug, ValueEnum, Display)]
pub enum Scene {
    Chapter6,
    Chapter8,
    Chapter9,
    Chapter10,
}

impl Scene {
    #[must_use]
    pub fn generate(&self) -> SceneData {
        match self {
            Self::Chapter6 => chapter6::generate_scene(),
            Self::Chapter8 => chapter8::generate_scene(),
            Self::Chapter9 => chapter9::generate_scene(),
            Self::Chapter10 => chapter10::generate_scene(),
        }
    }
}

/// `SceneData` contains all the information needed to render a given scene
/// including the `Camera` and all the objects and lights present in the
/// `World`.
#[derive(Clone, Debug, Constructor)]
pub struct SceneData {
    pub camera: Camera,
    pub world: World,
}

impl SceneData {
    #[must_use]
    pub fn render(&self, quiet: bool) -> Canvas {
        self.camera.render(&self.world, quiet)
    }
}
