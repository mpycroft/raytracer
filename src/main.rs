use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs::write,
};

use raytracer::{
    math::{Angle, Point, Transform, Vector},
    Camera, Colour, Material, PointLight, Sphere, World,
};

fn main() {
    let mut world = World::new();

    let floor_material =
        Material::new(Colour::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0);

    let mut floor_transform = Transform::from_scale(10.0, 0.01, 10.0);
    world.push_object(Sphere::new(floor_transform, floor_material));

    let mut wall_transform =
        floor_transform.rotate_x(Angle::from_radians(FRAC_PI_2));

    world.push_object(Sphere::new(
        wall_transform
            .clone()
            .rotate_y(Angle::from_radians(-FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material,
    ));
    world.push_object(Sphere::new(
        wall_transform
            .rotate_y(Angle::from_radians(FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material,
    ));

    world.push_object(Sphere::new(
        Transform::from_translate(-0.5, 1.0, 0.5),
        Material::new(Colour::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0),
    ));
    world.push_object(Sphere::new(
        Transform::from_scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material::new(Colour::new(0.5, 1.0, 0.1), 0.1, 0.7, 0.3, 200.0),
    ));
    world.push_object(Sphere::new(
        Transform::from_scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75),
        Material::new(Colour::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0),
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
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let image = camera.render(&world);

    write("image.ppm", image.to_ppm()).unwrap();
}
