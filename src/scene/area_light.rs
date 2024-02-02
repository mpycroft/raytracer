use std::f64::consts::FRAC_PI_3;

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
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_3));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 6.0, -1.0),
            &Point::new(0.0, 5.5, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Object::plane_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(0.7, 0.7, 0.7).into())
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .material(
                Material::builder().pattern(Colour::blue().into()).build(),
            )
            .transformation(
                Transformation::new()
                    .scale(2.0, 2.0, 2.0)
                    .translate(-1.0, 1.0, 7.0),
            )
            .build(),
    );
    world.add_object(
        Object::sphere_builder()
            .material(Material::builder().pattern(Colour::red().into()).build())
            .transformation(Transformation::new().translate(1.0, 1.0, 5.0))
            .build(),
    );

    world.add_light(Light::new_area(
        Point::new(-10.0, 5.0, -7.0),
        Vector::new(0.0, 4.0, 0.0),
        4,
        Vector::new(0.0, 0.0, 2.0),
        2,
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
