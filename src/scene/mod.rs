mod chapter10;

use clap::ValueEnum;
use derive_more::{Constructor, Display};
use raytracer::{Camera, Canvas, World};

/// `Scene` is a list of all the scenes we know about.
#[derive(Clone, Copy, Debug, ValueEnum, Display)]
pub enum Scene {
    Chapter10,
}

impl Scene {
    #[must_use]
    pub fn generate(&self) -> SceneData {
        match self {
            Scene::Chapter10 => chapter10::generate_scene(),
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
