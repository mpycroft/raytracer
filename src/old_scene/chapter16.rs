use std::f64::consts::FRAC_PI_3;

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, Operation, Pattern, World,
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
            Point::new(0.0, 4.0, -1.0),
            Point::new(0.0, 3.5, 0.0),
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
                            Colour::white().into(),
                            Colour::black().into(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .build(),
    );

    world.add_object(Object::new_csg(
        Operation::Difference,
        Object::cube_builder()
            .material(
                Material::builder().pattern(Colour::yellow().into()).build(),
            )
            .transformation(Transformation::new().translate(-4.0, 1.0, 14.0))
            .build(),
        Object::sphere_builder()
            .material(Material::builder().pattern(Colour::red().into()).build())
            .transformation(Transformation::new().translate(-3.5, 1.5, 13.5))
            .build(),
    ));

    world.add_object(Object::new_csg(
        Operation::Intersection,
        Object::sphere_builder()
            .material(Material::glass())
            .transformation(Transformation::new().translate(-4.0, 1.0, 10.0))
            .build(),
        Object::sphere_builder()
            .material(Material::glass())
            .transformation(Transformation::new().translate(-4.0, 1.0, 9.0))
            .build(),
    ));

    world.add_object(Object::new_csg(
        Operation::Union,
        Object::cube_builder()
            .material(
                Material::builder().pattern(Colour::green().into()).build(),
            )
            .transformation(Transformation::new().translate(4.0, 1.0, 14.0))
            .build(),
        Object::sphere_builder()
            .material(
                Material::builder().pattern(Colour::blue().into()).build(),
            )
            .transformation(Transformation::new().translate(4.0, 2.0, 14.0))
            .build(),
    ));

    world.add_object(Object::new_csg(
        Operation::Difference,
        Object::new_csg(
            Operation::Intersection,
            Object::sphere_builder()
                .material(
                    Material::builder()
                        .pattern(Colour::black().into())
                        .reflective(0.3)
                        .build(),
                )
                .transformation(
                    Transformation::new()
                        .scale(1.3, 1.3, 1.3)
                        .translate(0.0, 1.0, 5.0),
                )
                .build(),
            Object::cube_builder()
                .material(
                    Material::builder()
                        .pattern(Colour::new(0.4, 0.4, 0.4).into())
                        .specular(0.6)
                        .shininess(200.0)
                        .reflective(0.06)
                        .build(),
                )
                .transformation(
                    Transformation::new()
                        .rotate_y(Angle::from_degrees(50.0))
                        .translate(0.0, 1.0, 5.0),
                )
                .build(),
        ),
        Object::new_csg(
            Operation::Union,
            Object::new_csg(
                Operation::Union,
                Object::cylinder_builder(-2.0, 2.0, true)
                    .material(
                        Material::builder()
                            .pattern(Colour::green().into())
                            .build(),
                    )
                    .transformation(
                        Transformation::new()
                            .scale(0.5, 1.0, 0.5)
                            .translate(0.0, 1.0, 5.0),
                    )
                    .build(),
                Object::cylinder_builder(-2.0, 2.0, true)
                    .material(
                        Material::builder()
                            .pattern(Colour::red().into())
                            .build(),
                    )
                    .transformation(
                        Transformation::new()
                            .scale(0.5, 1.0, 0.5)
                            .rotate_x(Angle::from_degrees(90.0))
                            .rotate_y(-Angle::from_degrees(50.0))
                            .translate(0.0, 1.0, 5.0),
                    )
                    .build(),
            ),
            Object::cylinder_builder(-2.0, 2.0, true)
                .material(
                    Material::builder().pattern(Colour::blue().into()).build(),
                )
                .transformation(
                    Transformation::new()
                        .scale(0.5, 1.0, 0.5)
                        .rotate_x(Angle::from_degrees(90.0))
                        .rotate_y(Angle::from_degrees(50.0))
                        .translate(0.0, 1.0, 5.0),
                )
                .build(),
        ),
    ));

    world.add_light(Light::new_point(
        Point::new(-100.0, 100.0, -100.0),
        Colour::new(0.7, 0.7, 0.7),
    ));
    world.add_light(Light::new_point(
        Point::new(10.0, 10.0, -10.0),
        Colour::new(0.2, 0.2, 0.2),
    ));

    SceneData::new(camera, world)
}
