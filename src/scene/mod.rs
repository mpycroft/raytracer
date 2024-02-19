mod add;
mod define;
mod list;
mod material;
mod transformations;

use std::{collections::HashMap, fs::File, path::Path};

use anyhow::Result;
use derive_new::new;
use serde_yaml::{from_reader, Value};

use self::{
    add::Add, define::Define, list::List, material::Material,
    transformations::TransformationList,
};
use crate::{Camera, Light, World};

/// The `Data` struct holds the information for the scene as we parse it.
#[derive(Clone, Debug)]
struct Data {
    shapes: HashMap<String, Value>,
    materials: HashMap<String, Material>,
    transformations: HashMap<String, TransformationList>,
    camera: Option<Camera>,
    lights: Vec<Light>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            materials: HashMap::new(),
            transformations: HashMap::new(),
            camera: None,
            lights: Vec::new(),
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
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let list: List = from_reader(File::open(filename)?)?;

        let mut data = Data::new();
        list.parse(&mut data)?;

        // We have already checked that camera is Some when parsing list.
        let Some(camera) = data.camera else { unreachable!() };

        let mut world = World::new();
        world.lights = data.lights;

        Ok(Self { camera, world })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_3;

    use super::*;
    use crate::{
        math::{float::assert_approx_eq, Angle, Point, Transformation, Vector},
        Colour,
    };

    #[test]
    fn from_simple_yaml() {
        let s = Scene::from_file("src/scene/tests/simple.yaml").unwrap();

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
    }
}
