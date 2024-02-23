mod add;
mod define;
mod list;
mod material;
mod shapes;
mod transformations;

use std::{collections::HashMap, fs::File, io::Write, path::Path};

use anyhow::Result;
use derive_new::new;
use rand::Rng;
use serde_yaml::{from_reader, Value};

use self::{
    add::Add, define::Define, list::List, material::Material,
    transformations::TransformationList,
};
use crate::{Camera, Canvas, Light, Object, Output, World};

type HashValue = HashMap<String, Value>;

/// The `Data` struct holds the information for the scene as we parse it.
#[derive(Clone, Debug)]
struct Data {
    shapes: HashMap<String, Add>,
    materials: HashMap<String, Material>,
    transformations: HashMap<String, TransformationList>,
    camera: Option<Camera>,
    lights: Vec<Light>,
    objects: Vec<Object>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            materials: HashMap::new(),
            transformations: HashMap::new(),
            camera: None,
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
}

/// `Scene` contains all the information needed to render a given scene
/// including the `Camera` and all the objects and lights present in the
/// `World`.
#[derive(Clone, Debug, new)]
pub struct Scene {
    camera: Camera,
    world: World,
}

impl Scene {
    /// Load a scene from a Yaml file.
    ///
    /// # Errors
    ///
    /// Will return error if there are problems reading the file or parsing the
    /// data.
    pub fn from_file<P: AsRef<Path>>(filename: P, scale: f64) -> Result<Self> {
        let list: List = from_reader(File::open(filename)?)?;

        let mut data = Data::new();
        list.parse(&mut data)?;

        // We have already checked that camera is Some when parsing list.
        let Some(mut camera) = data.camera else { unreachable!() };
        camera.scale(scale);

        let mut world = World::new();
        world.lights = data.lights;
        world.objects = data.objects;

        Ok(Self { camera, world })
    }

    /// Render a scene to a `Canvas`.
    ///
    /// # Errors
    ///
    /// Returns an error if there are problems writing status messages.
    pub fn render<O: Write, R: Rng>(
        &self,
        depth: u32,
        single_threaded: bool,
        output: &mut Output<O>,
        rng: &mut R,
    ) -> Result<Canvas> {
        self.camera.render(&self.world, depth, single_threaded, output, rng)
    }

    #[must_use]
    pub const fn horizontal_size(&self) -> u32 {
        self.camera.horizontal_size()
    }

    #[must_use]
    pub const fn vertical_size(&self) -> u32 {
        self.camera.vertical_size()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_3;

    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;
    use crate::{
        math::{float::*, Angle, Point, Transformation, Vector},
        Colour,
    };

    #[test]
    fn from_simple_yaml() {
        let s = Scene::from_file("src/scene/tests/simple.yaml", 1.0).unwrap();

        assert_approx_eq!(
            s.camera,
            Camera::new(
                200,
                200,
                Angle(FRAC_PI_3),
                Transformation::view_transformation(
                    Point::new(2.0, 3.0, -5.0),
                    Point::new(2.0, 1.5, 0.0),
                    Vector::y_axis()
                )
            )
        );

        assert_eq!(s.world.lights.len(), 1);
        assert_approx_eq!(
            s.world.lights[0],
            Light::new_point(Point::new(-10.0, 10.0, -10.0), Colour::white())
        );

        s.render(
            5,
            true,
            &mut Output::<Vec<_>>::new_sink(),
            &mut Xoshiro256PlusPlus::seed_from_u64(0),
        )
        .unwrap();
    }

    #[test]
    fn test_scale() {
        let s = Scene::from_file("src/scene/tests/simple.yaml", 2.5).unwrap();

        assert_eq!(s.horizontal_size(), 500);
        assert_eq!(s.vertical_size(), 500);
    }
}
