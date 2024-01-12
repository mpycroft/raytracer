mod chapter10;
mod chapter11;
mod chapter12;
mod chapter6;
mod chapter8;
mod chapter9;

use anyhow::Result;
use clap::ValueEnum;
use derive_more::Display;
use derive_new::new;
use raytracer::{Camera, Canvas, World};

use crate::arguments::Arguments;

/// `Scene` is a list of all the scenes we know about.
#[derive(Clone, Copy, Debug, ValueEnum, Display)]
pub enum Scene {
    Chapter6,
    Chapter8,
    Chapter9,
    Chapter10,
    Chapter11,
    Chapter11Water,
    Chapter12,
}

impl Scene {
    #[must_use]
    pub fn generate(self, arguments: &Arguments) -> SceneData {
        match self {
            Self::Chapter6 => chapter6::generate_scene(arguments),
            Self::Chapter8 => chapter8::generate_scene(arguments),
            Self::Chapter9 => chapter9::generate_scene(arguments),
            Self::Chapter10 => chapter10::generate_scene(arguments),
            Self::Chapter11 => chapter11::generate_scene(arguments),
            Self::Chapter11Water => chapter11::generate_water_scene(arguments),
            Self::Chapter12 => chapter12::generate_scene(arguments),
        }
    }
}

/// `SceneData` contains all the information needed to render a given scene
/// including the `Camera` and all the objects and lights present in the
/// `World`.
#[derive(Clone, Debug, new)]
#[allow(clippy::module_name_repetitions)]
pub struct SceneData {
    pub camera: Camera,
    pub world: World,
}

impl SceneData {
    pub fn render(&self, depth: u32, quiet: bool) -> Result<Canvas> {
        self.camera.render(&self.world, depth, quiet)
    }
}
