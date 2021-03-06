use std::{
    f64::consts::{FRAC_PI_3, FRAC_PI_4},
    fs::write,
};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use raytracer::{
    math::{Angle, PerlinNoise, Point, Transform, Vector},
    Camera, Colour, Material, Object, Pattern, PointLight, World,
};

fn main() {
    let mut world = World::new();

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let floor_material = Material::new(
        Pattern::default_perturbed(
            PerlinNoise::new(&mut rng),
            Pattern::default_stripe(Colour::white(), Colour::red()),
            0.3,
        ),
        0.1,
        0.9,
        0.0,
        200.0,
    );

    world.push_object(Object::new_plane(
        Transform::default(),
        floor_material.clone(),
    ));

    let wall_transform = Transform::from_rotate_x(Angle::from_degrees(90.0));
    world.push_object(Object::new_plane(
        wall_transform
            .clone()
            .rotate_y(Angle::from_radians(-FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material.clone(),
    ));
    world.push_object(Object::new_plane(
        wall_transform
            .clone()
            .rotate_y(Angle::from_radians(FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        floor_material,
    ));

    world.push_object(Object::new_sphere(
        Transform::from_translate(-0.5, 1.0, 0.5),
        Material::new(
            Pattern::default_perturbed(
                PerlinNoise::new(&mut rng),
                Pattern::new_ring(
                    Transform::from_scale(0.3, 0.3, 0.3),
                    Colour::new(0.1, 1.0, 0.5),
                    Colour::white(),
                ),
                0.04,
            ),
            0.1,
            0.7,
            0.3,
            200.0,
        ),
    ));
    world.push_object(Object::new_sphere(
        Transform::from_scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material::new(
            Pattern::default_uniform(Colour::new(0.5, 1.0, 0.1)),
            0.1,
            0.7,
            0.3,
            200.0,
        ),
    ));
    world.push_object(Object::new_sphere(
        Transform::from_scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75),
        Material::new(
            Pattern::default_uniform(Colour::new(1.0, 0.8, 0.1)),
            0.1,
            0.7,
            0.3,
            200.0,
        ),
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
