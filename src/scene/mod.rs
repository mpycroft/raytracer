mod bounding_box;
mod chapter10;
mod chapter11;
mod chapter12;
mod chapter13;
mod chapter14;
mod chapter15;
mod chapter16;
mod chapter6;
mod chapter8;
mod chapter9;

use std::io::Write;

use anyhow::Result;
use clap::ValueEnum;
use derive_more::Display;
use derive_new::new;
use rand::Rng;
use raytracer::{Camera, Canvas, Output, World};

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
    Chapter13,
    Chapter14,
    Chapter14Spheres,
    Chapter15,
    Chapter16,
    BoundingBox,
}

impl Scene {
    #[must_use]
    pub fn generate<R: Rng>(
        self,
        arguments: &Arguments,
        rng: &mut R,
    ) -> SceneData {
        match self {
            Self::Chapter6 => chapter6::generate_scene(arguments),
            Self::Chapter8 => chapter8::generate_scene(arguments),
            Self::Chapter9 => chapter9::generate_scene(arguments),
            Self::Chapter10 => chapter10::generate_scene(arguments, rng),
            Self::Chapter11 => chapter11::generate_scene(arguments),
            Self::Chapter11Water => chapter11::generate_water_scene(arguments),
            Self::Chapter12 => chapter12::generate_scene(arguments),
            Self::Chapter13 => chapter13::generate_scene(arguments),
            Self::Chapter14 => chapter14::generate_scene(arguments),
            Self::Chapter14Spheres => {
                chapter14::generate_sphere_scene(arguments, rng)
            }
            Self::Chapter15 => chapter15::generate_scene(arguments),
            Self::Chapter16 => chapter16::generate_scene(arguments),
            Self::BoundingBox => bounding_box::generate_scene(arguments),
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
    pub fn render<O: Write>(
        &self,
        depth: u32,
        single_threaded: bool,
        output: &mut Output<O>,
    ) -> Result<Canvas> {
        self.camera.render(&self.world, depth, single_threaded, output)
    }
}
