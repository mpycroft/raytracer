use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, Pattern, PointLight, World,
};

use super::SceneData;

#[must_use]
pub fn generate_scene() -> SceneData {
    let camera = Camera::new(
        1000,
        500,
        Angle(FRAC_PI_3),
        Transformation::view_transformation(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(Object::new_plane(
        Transformation::new(),
        Material {
            pattern: Pattern::default_checker(
                Colour::white().into(),
                Colour::black().into(),
            ),
            specular: 0.0,
            reflective: 0.4,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_plane(
        Transformation::new()
            .rotate_x(Angle(FRAC_PI_2))
            .translate(0.0, 0.0, 5.0),
        Material {
            pattern: Colour::new(0.1, 0.1, 0.1).into(),
            reflective: 1.0,
            transparency: 0.0,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_glass_sphere(
        Transformation::new().translate(-0.5, 1.0, 0.5),
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(-0.5, 1.0, 0.5),
        Material {
            ambient: 0.0,
            diffuse: 0.0,
            reflective: 0.5,
            transparency: 1.0,
            refractive_index: 1.0,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.4, 0.4, 0.4).translate(2.5, 0.5, 1.5),
        Material {
            pattern: Colour::new(0.5, 1.0, 0.1).into(),
            diffuse: 0.7,
            specular: 0.3,
            reflective: 0.2,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.3, 0.3, 0.3).translate(1.3, 0.5, 0.5),
        Material {
            pattern: Colour::new(0.5, 0.4, 0.8).into(),
            diffuse: 0.7,
            specular: 0.4,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_glass_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
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
            reflective: 0.4,
            ..Default::default()
        },
        true,
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 5.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));

    SceneData::new(camera, world)
}

#[must_use]
pub fn generate_water_scene() -> SceneData {
    let camera = Camera::new(
        1000,
        500,
        Angle(FRAC_PI_3),
        Transformation::view_transformation(
            &Point::new(0.0, 0.5, -5.0),
            &Point::origin(),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(Object::new_plane(
        Transformation::new().translate(0.0, -5.0, 0.0),
        Material {
            pattern: Pattern::default_checker(
                Colour::green().into(),
                Colour::white().into(),
            ),
            specular: 0.0,
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_plane(
        Transformation::new()
            .rotate_x(Angle(FRAC_PI_2))
            .translate(0.0, 0.0, 100.0),
        Material {
            pattern: Pattern::default_checker(
                Colour::white().into(),
                Colour::black().into(),
            ),
            ..Default::default()
        },
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(0.0, -4.5, 20.0),
        Material { pattern: Colour::blue().into(), ..Default::default() },
        true,
    ));
    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(5.0, -4.5, 15.0),
        Material { pattern: Colour::green().into(), ..Default::default() },
        true,
    ));
    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(-4.0, -4.5, 10.0),
        Material { pattern: Colour::red().into(), ..Default::default() },
        true,
    ));

    world.add_object(Object::new_plane(
        Transformation::new().translate(0.0, -2.0, 0.0),
        Material {
            reflective: 0.1,
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        },
        false,
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 5.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));

    SceneData::new(camera, world)
}
