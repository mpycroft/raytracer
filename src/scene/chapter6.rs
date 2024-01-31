use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, PointLight, Shape, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(500);
    let vertical_size = arguments.height.unwrap_or(500);
    let field_of_view = arguments.fov.unwrap_or(Angle(0.5));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 0.0, -5.0),
            &Point::origin(),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(
        Shape::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(1.0, 0.2, 1.0).into())
                    .build(),
            )
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
