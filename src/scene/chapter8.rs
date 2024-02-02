use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(500);
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_3));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    let floor_material = Material::builder()
        .pattern(Colour::new(1.0, 0.9, 0.9).into())
        .specular(0.0)
        .build();

    let floor_transformation = Transformation::new().scale(10.0, 0.01, 10.0);

    world.add_object(
        Object::sphere_builder()
            .transformation(floor_transformation)
            .material(floor_material.clone())
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                floor_transformation
                    .rotate_x(Angle(FRAC_PI_2))
                    .rotate_y(Angle(-FRAC_PI_4))
                    .translate(0.0, 0.0, 5.0),
            )
            .material(floor_material.clone())
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                floor_transformation
                    .rotate_x(Angle(FRAC_PI_2))
                    .rotate_y(Angle(FRAC_PI_4))
                    .translate(0.0, 0.0, 5.0),
            )
            .material(floor_material)
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(Transformation::new().translate(-0.5, 1.0, 0.5))
            .material(
                Material::builder()
                    .pattern(Colour::new(0.1, 1.0, 0.5).into())
                    .diffuse(0.7)
                    .specular(0.3)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(1.5, 0.5, -0.5),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(0.5, 1.0, 0.1).into())
                    .diffuse(0.7)
                    .specular(0.3)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.33, 0.33, 0.33)
                    .translate(-1.5, 0.33, -0.75),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(1.0, 0.8, 0.1).into())
                    .diffuse(0.7)
                    .specular(0.3)
                    .build(),
            )
            .build(),
    );

    world.add_light(Light::new_point(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
