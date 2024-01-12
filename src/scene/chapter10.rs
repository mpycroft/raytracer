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

    world.add_object(Object::new_plane(
        Transformation::new(),
        Material::builder()
            .pattern(Pattern::default_perturbed(
                0.15,
                Pattern::default_blend(
                    Pattern::new_stripe(
                        Transformation::new().scale(0.5, 0.5, 0.5),
                        Colour::green().into(),
                        Colour::white().into(),
                    ),
                    Pattern::new_stripe(
                        Transformation::new()
                            .rotate_y(Angle(FRAC_PI_2))
                            .scale(0.5, 0.5, 0.5),
                        Colour::green().into(),
                        Colour::white().into(),
                    ),
                ),
            ))
            .build(),
        true,
    ));
    world.add_object(Object::new_plane(
        Transformation::new()
            .rotate_x(Angle(FRAC_PI_2))
            .translate(0.0, 0.0, 10.0),
        Material::builder()
            .pattern(Pattern::default_gradient(
                Colour::purple().into(),
                Colour::yellow().into(),
            ))
            .build(),
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().translate(-0.5, 1.0, 0.5),
        Material::builder()
            .pattern(Pattern::new_ring(
                Transformation::new()
                    .rotate_x(Angle::from_degrees(70.0))
                    .rotate_z(Angle::from_degrees(-40.0))
                    .scale(0.2, 0.2, 0.2),
                Colour::blue().into(),
                Colour::cyan().into(),
            ))
            .diffuse(0.7)
            .specular(0.3)
            .build(),
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material::builder()
            .pattern(Pattern::new_checker(
                Transformation::new().scale(0.3, 0.3, 0.3),
                Colour::green().into(),
                Colour::cyan().into(),
            ))
            .diffuse(0.7)
            .specular(0.3)
            .build(),
        true,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
        Material::builder()
            .pattern(Pattern::new_radial_gradient(
                Transformation::new().scale(0.2, 0.2, 0.2),
                Colour::white().into(),
                Colour::black().into(),
            ))
            .diffuse(0.7)
            .specular(0.3)
            .build(),
        true,
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::new(0.8, 0.8, 0.8),
    ));
    world.add_light(PointLight::new(
        Point::new(10.0, 10.0, -10.0),
        Colour::new(0.1, 0.1, 0.5),
    ));

    SceneData::new(camera, world)
}
