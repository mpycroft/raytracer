use std::f64::consts::FRAC_PI_2;

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(1000);
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_2));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            Point::origin(),
            Point::new(0.5, -0.2, 1.0),
            Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Object::cube_builder()
            .transformation(Transformation::new().scale(5.0, 5.0, 5.0))
            .material(
                Material::builder()
                    .pattern(Colour::new(0.4, 0.4, 0.1).into())
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::cube_builder()
            .transformation(
                Transformation::new()
                    .scale(5.0, 0.1, 2.0)
                    .translate(0.0, -2.0, 4.0),
            )
            .material(Material::builder().pattern(Colour::red().into()).build())
            .build(),
    );
    world.add_object(
        Object::cube_builder()
            .transformation(
                Transformation::new()
                    .scale(0.2, 0.7, 0.2)
                    .translate(0.0, -2.0, 3.0),
            )
            .material(
                Material::builder().pattern(Colour::green().into()).build(),
            )
            .build(),
    );
    world.add_object(
        Object::cube_builder()
            .transformation(
                Transformation::new()
                    .rotate_y(Angle::from_degrees(30.0))
                    .scale(0.2, 0.7, 0.2)
                    .translate(2.0, -2.0, 3.0),
            )
            .material(
                Material::builder().pattern(Colour::blue().into()).build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(1.0, -1.5, 4.0),
            )
            .material(Material::glass())
            .build(),
    );

    world.add_object(
        Object::cube_builder()
            .transformation(
                Transformation::new()
                    .scale(0.1, 1.5, 3.0)
                    .translate(5.0, 0.0, 3.0),
            )
            .material(
                Material::builder()
                    .ambient(0.0)
                    .diffuse(0.0)
                    .specular(0.3)
                    .reflective(1.0)
                    .build(),
            )
            .build(),
    );

    world.add_light(Light::new_point(
        Point::new(-4.0, 4.0, -4.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
