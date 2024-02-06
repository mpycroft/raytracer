use std::f64::consts::FRAC_PI_4;

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(800);
    let vertical_size = arguments.height.unwrap_or(320);
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_4));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(-3.0, 1.0, 2.5),
            &Point::new(0.0, 0.5, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_light(Light::new_area(
        Point::new(-1.0, 2.0, 4.0),
        Vector::new(2.0, 0.0, 0.0),
        10,
        Vector::new(0.0, 2.0, 0.0),
        10,
        Colour::new(1.5, 1.5, 1.5),
    ));

    world.add_object(
        Object::cube_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(1.5, 1.5, 1.5).into())
                    .ambient(1.0)
                    .diffuse(0.0)
                    .specular(0.0)
                    .build(),
            )
            .transformation(
                Transformation::new()
                    .scale(1.0, 1.0, 0.01)
                    .translate(0.0, 3.0, 4.0),
            )
            .casts_shadow(false)
            .build(),
    );

    world.add_object(
        Object::plane_builder()
            .material(
                Material::builder()
                    .pattern(Colour::white().into())
                    .ambient(0.025)
                    .diffuse(0.67)
                    .specular(0.0)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::red().into())
                    .ambient(0.1)
                    .specular(0.0)
                    .diffuse(0.6)
                    .reflective(0.3)
                    .build(),
            )
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.5, 0.5, 0.0),
            )
            .build(),
    );
    world.add_object(
        Object::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(0.5, 0.5, 1.0).into())
                    .ambient(0.1)
                    .specular(0.0)
                    .diffuse(0.6)
                    .reflective(0.3)
                    .build(),
            )
            .transformation(
                Transformation::new()
                    .scale(0.33, 0.33, 0.33)
                    .translate(-0.25, 0.33, 0.0),
            )
            .build(),
    );

    SceneData::new(camera, world)
}
