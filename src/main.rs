// Ignore pedantic lints in our temp binary code until we actually start writing
// real raytracer code here.
#![allow(clippy::pedantic)]

mod arguments;

use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs::write,
    io::Error,
};

use clap::Parser;
use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Material, Object, PointLight, World,
};

use crate::arguments::Arguments;

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();

    if !arguments.quiet {
        print!("Generating scene...");
    }

    let mut world = World::new();

    let material = Material {
        colour: Colour::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    world.add_object(Object::new_sphere(
        Transformation::new().scale(10.0, 0.01, 10.0),
        material,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(Angle(FRAC_PI_2))
            .rotate_y(Angle(-FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        material,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(Angle(FRAC_PI_2))
            .rotate_y(Angle(FRAC_PI_4))
            .translate(0.0, 0.0, 5.0),
        material,
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .translate(-0.5, 1.0, 0.5)
            .shear(0.0, 0.9, 0.0, 0.0, 0.0, 0.0),
        Material {
            colour: Colour::new(0.1, 0.1, 1.0),
            diffuse: 0.7,
            specular: 0.9,
            ..Default::default()
        },
    ));

    world.add_object(Object::new_sphere(
        Transformation::new().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
        Material {
            colour: Colour::new(0.1, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    ));

    world.add_object(Object::new_sphere(
        Transformation::new()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
        Material {
            colour: Colour::new(1.0, 0.1, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    ));

    world.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));
    world.add_light(PointLight::new(
        Point::new(10.0, 10.0, -10.0),
        Colour::new(0.5, 0.3, 0.3),
    ));

    let camera = Camera::new(
        1000,
        500,
        Angle(FRAC_PI_3),
        Transformation::view_transformation(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    if !arguments.quiet {
        println!("done");
    }

    let canvas = camera.render(&world, arguments.quiet);

    if !arguments.quiet {
        println!("Writing to file {}", arguments.out);
    }

    write(arguments.out, canvas.to_ppm())
}
