mod add;
mod define;
mod list;
mod material;
mod shapes;
mod transformations;

use std::{
    collections::HashMap, f64::consts::FRAC_PI_3, fs::File, io::Write,
    path::Path,
};

use anyhow::Result;
use derive_new::new;
use rand::prelude::*;
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
    pub fn from_file<P, R>(filename: P, scale: f64, rng: &mut R) -> Result<Self>
    where
        P: AsRef<Path>,
        R: Rng,
    {
        let list: List = from_reader(File::open(filename)?)?;

        let mut data = Data::new();
        list.parse(&mut data, rng)?;

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

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn generate_random_spheres<R: Rng>(scale: f64, rng: &mut R) -> Self {
        use crate::{
            math::{Angle, Point, Transformation, Vector},
            Colour, Material, Pattern,
        };

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let camera = Camera::new(
            (1000.0 * scale) as u32,
            (800.0 * scale) as u32,
            Angle(FRAC_PI_3),
            Transformation::view_transformation(
                Point::new(0.0, 2.0, -1.0),
                Point::new(0.0, 1.8, 0.0),
                Vector::y_axis(),
            ),
        );

        let mut world = World::new();

        world.add_object(
            Object::plane_builder()
                .material(
                    Material::builder()
                        .pattern(
                            Pattern::checker_builder(
                                Colour::new(0.5, 0.5, 0.4).into(),
                                Colour::new(0.5, 0.4, 0.3).into(),
                            )
                            .build(),
                        )
                        .build(),
                )
                .build(),
        );

        let mut generate_spheres = |num_spheres, min, max| {
            let mut spheres = Vec::with_capacity(num_spheres);
            let mut locations: Vec<Point> = Vec::with_capacity(num_spheres);

            for _ in 0..num_spheres {
                let check_location = |point: &Point| {
                    for location in &locations {
                        if (location.x - point.x).abs() < 0.6 {
                            return false;
                        }
                        if (location.z - point.z).abs() < 0.6 {
                            return false;
                        }
                    }

                    true
                };

                let mut location = Point::new(
                    rng.gen_range(min..=max),
                    0.5,
                    rng.gen_range(min..=max),
                );

                while !check_location(&location) {
                    location = Point::new(
                        rng.gen_range(min..=max),
                        0.5,
                        rng.gen_range(min..=max),
                    );
                }

                locations.push(location);

                let material = if rng.gen_range(0.0..=1.0) < 0.1 {
                    Material::glass()
                } else {
                    let reflective = if rng.gen_range(0.0..=1.0) < 0.4 {
                        0.0
                    } else {
                        rng.gen_range(0.0..=1.0)
                    };

                    Material::builder()
                        .pattern(
                            Colour::new(
                                rng.gen_range(0.0..=1.0),
                                rng.gen_range(0.0..=1.0),
                                rng.gen_range(0.0..=1.0),
                            )
                            .into(),
                        )
                        .ambient(rng.gen_range(0.0..=1.0))
                        .diffuse(rng.gen_range(0.0..=1.0))
                        .specular(rng.gen_range(0.0..=1.0))
                        .shininess(rng.gen_range(0.0..=250.0))
                        .reflective(reflective)
                        .build()
                };

                spheres.push(
                    Object::sphere_builder()
                        .transformation(
                            Transformation::new()
                                .scale(0.5, 0.5, 0.5)
                                .translate(location.x, location.y, location.z),
                        )
                        .material(material)
                        .build(),
                );
            }

            spheres
        };

        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(20, -10.0, 10.0))
                .transformation(
                    Transformation::new().translate(-10.0, 0.0, 35.0),
                )
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(20, -10.0, 10.0))
                .transformation(
                    Transformation::new().translate(10.0, 0.0, 35.0),
                )
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(20, -10.0, 10.0))
                .transformation(
                    Transformation::new().translate(-8.0, 0.0, 25.0),
                )
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(20, -10.0, 10.0))
                .transformation(Transformation::new().translate(8.0, 0.0, 25.0))
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(10, -5.0, 5.0))
                .transformation(
                    Transformation::new().translate(-5.0, 0.0, 10.0),
                )
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(10, -5.0, 5.0))
                .transformation(Transformation::new().translate(5.0, 0.0, 10.0))
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(10, -5.0, 5.0))
                .transformation(Transformation::new().translate(-5.0, 0.0, 0.0))
                .build(),
        );
        world.add_object(
            Object::group_builder()
                .set_objects(generate_spheres(10, -5.0, 5.0))
                .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
                .build(),
        );

        world.add_light(Light::new_point(
            Point::new(-100.0, 100.0, -100.0),
            Colour::new(0.5, 0.5, 0.5),
        ));
        world.add_light(Light::new_point(
            Point::new(100.0, 100.0, 100.0),
            Colour::new(0.5, 0.5, 0.5),
        ));

        Self { camera, world }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_3;

    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;
    use crate::{
        math::{float::*, Angle, Point, Transformation, Vector},
        Colour,
    };

    #[test]
    fn from_simple_yaml() {
        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        let s = Scene::from_file("src/scene/tests/simple.yaml", 1.0, &mut r)
            .unwrap();

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

        s.render(5, true, &mut Output::<Vec<_>>::new_sink(), &mut r).unwrap();
    }

    #[test]
    fn test_scale() {
        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        let s = Scene::from_file("src/scene/tests/simple.yaml", 2.5, &mut r)
            .unwrap();

        assert_eq!(s.horizontal_size(), 500);
        assert_eq!(s.vertical_size(), 500);
    }

    #[test]
    fn sphere_scene() {
        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        let s = Scene::generate_random_spheres(0.1, &mut r);

        s.render(5, true, &mut Output::<Vec<_>>::new_sink(), &mut r).unwrap();
    }
}
