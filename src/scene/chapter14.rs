use std::f64::consts::FRAC_PI_3;

use rand::Rng;
use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, Pattern, PointLight, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate_sphere_scene<R: Rng>(
    arguments: &Arguments,
    rng: &mut R,
) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(800);
    let field_of_view = arguments.fov.unwrap_or(Angle(FRAC_PI_3));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 2.0, -1.0),
            &Point::new(0.0, 1.8, 0.0),
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
                            Colour::new(0.5, 0.5, 0.4).into(),
                            Colour::new(0.5, 0.4, 0.3).into(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .build(),
    );

    let mut generate_spheres = |num_spheres, min, max| {
        let mut spheres = Vec::with_capacity(num_spheres);
        let mut locations: Vec<Point> = Vec::with_capacity(num_spheres);

        for _ in 0..num_spheres {
            let check_location = |point: &Point| {
                for location in &locations {
                    if (location.x - point.x).abs() < 0.6 {
                        return false;
                    }
                    if (location.z - point.z).abs() < 0.6 {
                        return false;
                    }
                }

                true
            };

            let mut location = Point::new(
                rng.gen_range(min..=max),
                0.5,
                rng.gen_range(min..=max),
            );

            while !check_location(&location) {
                location = Point::new(
                    rng.gen_range(min..=max),
                    0.5,
                    rng.gen_range(min..=max),
                );
            }

            locations.push(location);

            let material = if rng.gen_range(0.0..=1.0) < 0.1 {
                Material::glass()
            } else {
                let reflective = if rng.gen_range(0.0..=1.0) < 0.4 {
                    0.0
                } else {
                    rng.gen_range(0.0..=1.0)
                };

                Material::builder()
                    .pattern(
                        Colour::new(
                            rng.gen_range(0.0..=1.0),
                            rng.gen_range(0.0..=1.0),
                            rng.gen_range(0.0..=1.0),
                        )
                        .into(),
                    )
                    .ambient(rng.gen_range(0.0..=1.0))
                    .diffuse(rng.gen_range(0.0..=1.0))
                    .specular(rng.gen_range(0.0..=1.0))
                    .shininess(rng.gen_range(0.0..=250.0))
                    .reflective(reflective)
                    .build()
            };

            spheres.push(
                Object::sphere_builder()
                    .transformation(
                        Transformation::new()
                            .scale(0.5, 0.5, 0.5)
                            .translate(location.x, location.y, location.z),
                    )
                    .material(material)
                    .build(),
            );
        }

        spheres
    };

    world.add_object(
        Object::group_builder(generate_spheres(20, -10.0, 10.0))
            .transformation(Transformation::new().translate(-10.0, 0.0, 35.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(20, -10.0, 10.0))
            .transformation(Transformation::new().translate(10.0, 0.0, 35.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(20, -10.0, 10.0))
            .transformation(Transformation::new().translate(-8.0, 0.0, 25.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(20, -10.0, 10.0))
            .transformation(Transformation::new().translate(8.0, 0.0, 25.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(10, -5.0, 5.0))
            .transformation(Transformation::new().translate(-5.0, 0.0, 10.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(10, -5.0, 5.0))
            .transformation(Transformation::new().translate(5.0, 0.0, 10.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(10, -5.0, 5.0))
            .transformation(Transformation::new().translate(-5.0, 0.0, 0.0))
            .build(),
    );
    world.add_object(
        Object::group_builder(generate_spheres(10, -5.0, 5.0))
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build(),
    );

    world.add_light(PointLight::new(
        Point::new(-100.0, 100.0, -100.0),
        Colour::new(0.5, 0.5, 0.5),
    ));
    world.add_light(PointLight::new(
        Point::new(100.0, 100.0, 100.0),
        Colour::new(0.5, 0.5, 0.5),
    ));

    SceneData::new(camera, world)
}
