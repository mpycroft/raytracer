use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, PointLight, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(500);

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        Angle(FRAC_PI_3),
        Transformation::view_transformation(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    let floor_material = Material {
        pattern: Colour::new(1.0, 0.9, 0.9).into(),
        specular: 0.0,
        ..Default::default()
    };

    let floor_transformation = Transformation::new().scale(10.0, 0.01, 10.0);

    world.add_object(Object::new_sphere(
        floor_transformation,
        floor_material.clone(),
        true,
    ));

    world.add_object(Object::new_sphere(
        floor_transformation
            .rotate_x(Angle(FRAC_PI_2))
            .rotate_y(Angle(-FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material.clone(),
        true,
    ));

    world.add_object(Object::new_sphere(
        floor_transformation
            .rotate_x(Angle(FRAC_PI_2))
            .rotate_y(Angle(FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material,
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().translate(-0.5, 1.0, 0.5),
        Material {
            pattern: Colour::new(0.1, 1.0, 0.5).into(),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material {
            pattern: Colour::new(0.5, 1.0, 0.1).into(),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
        Material {
            pattern: Colour::new(1.0, 0.8, 0.1).into(),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
        true,
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
