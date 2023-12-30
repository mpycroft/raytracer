use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, PointLight, World,
};

use super::SceneData;

#[must_use]
pub fn generate_scene() -> SceneData {
    let camera = Camera::new(
        500,
        500,
        Angle(0.5),
        Transformation::view_transformation(
            &Point::new(0.0, 0.0, -5.0),
            &Point::origin(),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_object(Object::new_sphere(
        Transformation::new(),
        Material {
            pattern: Colour::new(1.0, 0.2, 1.0).into(),
            ..Default::default()
        },
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    SceneData::new(camera, world)
}
