use std::{f64::consts::FRAC_PI_3, fs::write, io};

use raytracer::{
    math::{Angle, Point, Transform, Vector},
    Camera, Colour, Material, Object, Pattern, PointLight, World,
};

fn main() -> io::Result<()> {
    let mut world = World::new();

    world.push_object(Object::new_plane(
        Transform::from_rotate_x(Angle::from_degrees(90.0))
            .translate(0.0, 0.0, 70.0),
        Material {
            pattern: Pattern::default_checker(
                Colour::new(0.8, 0.8, 0.8),
                Colour::new(0.3, 0.3, 0.3),
            ),
            ..Default::default()
        },
    ));

    world.push_object(Object::new_plane(
        Transform::from_translate(0.0, -20.0, 0.0),
        Material {
            pattern: Pattern::default_checker(
                Colour::new(0.1, 0.7, 0.3),
                Colour::new(0.5, 0.3, 0.0),
            ),
            ..Default::default()
        },
    ));

    world.push_object(Object::new_plane(
        Transform::from_translate(0.0, -10.0, 0.0),
        Material {
            pattern: Pattern::default_uniform(Colour::new(0.5, 0.5, 0.5)),
            ambient: 0.5,
            reflective: 0.4,
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        },
    ));

    world.push_light(PointLight::new(
        Colour::white(),
        Point::new(-10.0, 10.0, -10.0),
    ));

    let camera = Camera::new(
        1000,
        500,
        FRAC_PI_3,
        Transform::view_transform(
            &Point::new(0.0, 2.0, -5.0),
            &Point::new(0.0, 0.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let image = camera.render(&world, 10);

    write("image.ppm", image.to_ppm())?;

    Ok(())
}
