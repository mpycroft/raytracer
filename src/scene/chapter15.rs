use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

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
        Object::from_obj_file("obj/teapot.obj")
            .unwrap()
            .transformation(
                Transformation::new()
                    .rotate_x(-Angle(FRAC_PI_2))
                    .scale(0.3, 0.3, 0.3)
                    .translate(-0.4, 0.0, 7.0),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(0.0, 0.4, 0.6).into())
                    .shininess(100.0)
                    .reflective(0.05)
                    .build(),
            )
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-100.0, 50.0, -50.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
