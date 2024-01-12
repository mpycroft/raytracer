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

    world.add_object(
        Object::plane_builder()
            .material(
                Material::builder()
                    .pattern(Pattern::default_checker(
                        Colour::white().into(),
                        Colour::black().into(),
                    ))
                    .specular(0.0)
                    .reflective(0.4)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::plane_builder()
            .transformation(
                Transformation::new()
                    .rotate_x(Angle(FRAC_PI_2))
                    .translate(0.0, 0.0, 5.0),
            )
            .material(
                Material::builder()
                    .ambient(0.0)
                    .diffuse(0.0)
                    .reflective(1.0)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::glass_sphere_builder()
            .transformation(Transformation::new().translate(-0.5, 1.0, 0.5))
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(-0.5, 1.0, 0.5),
            )
            .material(
                Material::builder()
                    .ambient(0.0)
                    .diffuse(0.0)
                    .reflective(0.8)
                    .transparency(1.0)
                    .refractive_index(1.0)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.4, 0.4, 0.4)
                    .translate(2.5, 0.5, 1.5),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(0.5, 1.0, 0.1).into())
                    .diffuse(0.7)
                    .specular(0.3)
                    .reflective(0.2)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.3, 0.3, 0.3)
                    .translate(1.3, 0.5, 1.0),
            )
            .material(
                Material::builder()
                    .pattern(Colour::new(0.5, 0.4, 0.8).into())
                    .diffuse(0.7)
                    .specular(0.4)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::glass_sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(1.5, 0.5, -0.5),
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
                    .reflective(0.4)
                    .build(),
            )
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-10.0, 5.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));

    SceneData::new(camera, world)
}

#[must_use]
pub fn generate_water_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(500);
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_3));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 0.5, -5.0),
            &Point::origin(),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Object::plane_builder()
            .transformation(Transformation::new().translate(0.0, -5.0, 0.0))
            .material(
                Material::builder()
                    .pattern(Pattern::default_checker(
                        Colour::green().into(),
                        Colour::white().into(),
                    ))
                    .specular(0.0)
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::plane_builder()
            .transformation(
                Transformation::new()
                    .rotate_x(Angle(FRAC_PI_2))
                    .translate(0.0, 0.0, 100.0),
            )
            .material(
                Material::builder()
                    .pattern(Pattern::default_checker(
                        Colour::white().into(),
                        Colour::black().into(),
                    ))
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.0, -4.5, 20.0),
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
                    .translate(5.0, -4.5, 15.0),
            )
            .material(
                Material::builder().pattern(Colour::green().into()).build(),
            )
            .build(),
    );
    world.add_object(
        Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 0.5, 0.5)
                    .translate(-4.0, -4.5, 10.0),
            )
            .material(Material::builder().pattern(Colour::red().into()).build())
            .build(),
    );

    world.add_object(
        Object::plane_builder()
            .transformation(Transformation::new().translate(0.0, -2.0, 0.0))
            .material(
                Material::builder()
                    .reflective(0.1)
                    .transparency(1.0)
                    .refractive_index(1.5)
                    .build(),
            )
            .casts_shadow(false)
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-10.0, 5.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));

    SceneData::new(camera, world)
}
