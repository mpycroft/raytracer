// Ignore pedantic lints in our temp binary code until we actually start writing
// real raytracer code here.
#![allow(clippy::pedantic)]

use std::{f64::consts::FRAC_PI_6, fs::write, io::Error};

use raytracer::{
    math::{Point, Transformable, Transformation},
    Canvas, Colour,
};

fn main() -> Result<(), Error> {
    let mut canvas = Canvas::new(500, 500);

    let transform = Transformation::new()
        .scale(200.0, 200.0, 0.0)
        .translate(250.0, 250.0, 0.0);

    let point = Point::new(0.0, 1.0, 0.0);

    for count in 0..12 {
        let radians = count as f64 * FRAC_PI_6;

        let pixel = point
            .apply(&Transformation::new().rotate_z(radians).extend(&transform));

        canvas.write_pixel(
            pixel.x as usize,
            pixel.y as usize,
            &Colour::white(),
        );
    }

    write("image.ppm", canvas.to_ppm())
}
