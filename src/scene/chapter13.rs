use std::f64::{consts::FRAC_PI_3, INFINITY};

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, PointLight, World,
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
            &Point::new(0.0, 0.0, -1.0),
            &Point::origin(),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Object::cylinder_builder(-INFINITY, INFINITY)
            .transformation(Transformation::new().translate(0.0, 0.0, 15.0))
            .material(Material::builder().pattern(Colour::red().into()).build())
            .build(),
    );
    world.add_object(
        Object::cylinder_builder(-INFINITY, INFINITY)
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
