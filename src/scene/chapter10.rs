use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

use rand::Rng;
use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, Pattern, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate_scene<R: Rng>(arguments: &Arguments, rng: &mut R) -> SceneData {
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
                    .pattern(
                        Pattern::perturbed_builder(
                            0.15,
                            Pattern::blend_builder(
                                Pattern::stripe_builder(
                                    Colour::green().into(),
                                    Colour::white().into(),
                                )
                                .transformation(
                                    Transformation::new().scale(0.5, 0.5, 0.5),
                                )
                                .build(),
                                Pattern::stripe_builder(
                                    Colour::green().into(),
                                    Colour::white().into(),
                                )
                                .transformation(
                                    Transformation::new()
                                        .rotate_y(Angle(FRAC_PI_2))
                                        .scale(0.5, 0.5, 0.5),
                                )
                                .build(),
                            )
                            .build(),
                            rng,
                        )
                        .build(),
                    )
                    .build(),
            )
            .build(),
    );
    world.add_object(
        Object::plane_builder()
            .transformation(
                Transformation::new()
                    .rotate_x(Angle(FRAC_PI_2))
                    .translate(0.0, 0.0, 10.0),
            )
            .material(
                Material::builder()
                    .pattern(
                        Pattern::gradient_builder(
                            Colour::purple().into(),
                            Colour::yellow().into(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .build(),
    );

    world.add_object(
        Object::sphere_builder()
            .transformation(Transformation::new().translate(-0.5, 1.0, 0.5))
            .material(
                Material::builder()
                    .pattern(
                        Pattern::ring_builder(
                            Colour::blue().into(),
                            Colour::cyan().into(),
                        )
                        .transformation(
                            Transformation::new()
                                .rotate_x(Angle::from_degrees(70.0))
                                .rotate_z(Angle::from_degrees(-40.0))
                                .scale(0.2, 0.2, 0.2),
                        )
                        .build(),
                    )
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
                    .pattern(
                        Pattern::checker_builder(
                            Colour::green().into(),
                            Colour::cyan().into(),
                        )
                        .transformation(
                            Transformation::new().scale(0.3, 0.3, 0.3),
                        )
                        .build(),
                    )
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
                    .pattern(
                        Pattern::radial_gradient_builder(
                            Colour::white().into(),
                            Colour::black().into(),
                        )
                        .transformation(
                            Transformation::new().scale(0.2, 0.2, 0.2),
                        )
                        .build(),
                    )
                    .diffuse(0.7)
                    .specular(0.3)
                    .build(),
            )
            .build(),
    );

    world.add_light(Light::new_point(
        Point::new(-10.0, 10.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));
    world.add_light(Light::new_point(
        Point::new(10.0, 10.0, -10.0),
        Colour::new(0.1, 0.1, 0.5),
    ));

    SceneData::new(camera, world)
}
