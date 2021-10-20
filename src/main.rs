use raytracer::{
    math::{Matrix, Point, Ray},
    Canvas, Colour, Intersectable, Material, Sphere,
};
use std::fs::write;

fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let origin = Point::new(0.0, 0.0, -5.0);

    let wall_size = 7.0;
    let wall_z = 10.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let colour = Colour::new(1.0, 0.0, 0.0);

    let sphere = Sphere::new(
        Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            * Matrix::scale(0.5, 1.0, 1.0),
        Material::default(),
    );

    for y in 0..(canvas_pixels - 1) {
        let world_y = half - pixel_size * y as f64;

        for x in 0..(canvas_pixels - 1) {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(origin, (position - origin).normalise());

            if let Some(list) = sphere.intersect(&ray) {
                if list.hit().is_some() {
                    canvas.write_pixel(x, y, colour);
                }
            }
        }
    }

    write("image.ppm", canvas.to_ppm()).unwrap();
}
