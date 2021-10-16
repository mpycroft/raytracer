use raytracer::{
    math::{Matrix, Point},
    Canvas, Colour,
};
use std::{f64::consts::PI, fs::write};

fn main() {
    let mut canvas = Canvas::new(500, 500);

    let translate = Matrix::translate(250.0, 250.0, 0.0);
    let scale = Matrix::scale(200.0, 200.0, 200.0);

    let point = Point::new(0.0, 1.0, 0.0);

    for count in 0..12 {
        let rotate = Matrix::rotate_z((count as f64 * (2.0 * PI)) / 12.0);

        let pixel = translate * scale * rotate * point;

        canvas.write_pixel(
            pixel.x as usize,
            pixel.y as usize,
            Colour::new(1.0, 1.0, 1.0),
        );
    }

    write("image.ppm", canvas.to_ppm()).unwrap();
}
