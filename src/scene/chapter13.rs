use std::f64::{consts::FRAC_PI_3, INFINITY};

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, Pattern, PointLight, World,
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
            &Point::new(0.0, 5.0, -1.0),
            &Point::new(0.0, 4.5, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Object::plane_builder()
            .material(
                Material::builder()
                    .pattern(
                        Pattern::checker_builder(
                            Colour::white().into(),
                            Colour::black().into(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::cylinder_builder(0.0, 1.0, false)
            .transformation(
                Transformation::new()
                    .scale(5.0, 1.0, 5.0)
                    .translate(0.0, 0.0, 10.0),
            )
            .material(Material::builder().pattern(Colour::red().into()).build())
            .build(),
    );

    world.add_object(
        Object::cylinder_builder(0.0, 1.0, false)
            .transformation(
                Transformation::new()
                    .scale(3.5, 1.0, 3.5)
                    .translate(0.0, 0.0, 10.0),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(0.4, 0.5, 0.2).into())
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::cylinder_builder(0.0, 2.0, true)
            .transformation(Transformation::new().translate(0.0, 0.0, 10.0))
            .material(
                Material::builder().pattern(Colour::blue().into()).build(),
            )
            .build(),
    );

    world.add_object(
        Object::cylinder_builder(-INFINITY, INFINITY, false)
            .transformation(Transformation::new().translate(5.0, 0.0, 20.0))
            .material(
                Material::builder().pattern(Colour::green().into()).build(),
            )
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
