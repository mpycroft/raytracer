// Ignore pedantic lints in our temp binary code until we actually start writing
// real raytracer code here.
#![allow(clippy::pedantic)]

use std::{fs::write, io::Error};

use raytracer::{
    intersect::Intersectable,
    math::{Point, Ray, Transformation},
    Canvas, Colour, Material, PointLight, Sphere,
};

fn main() -> Result<(), Error> {
    let canvas_pixels = 500;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let origin = Point::new(0.0, 0.0, -5.0);

    let wall_size = 7.0;
    let wall_z = 10.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let sphere = Sphere::new(
        Transformation::new().scale(0.8, 1.2, 1.0).rotate_z(0.7),
        Material {
            colour: Colour::new(0.5, 0.2, 0.6),
            ambient: 0.4,
            diffuse: 0.4,
            ..Default::default()
        },
    );

    let light = PointLight::new(
        Point::new(10.0, 10.0, -10.0),
        Colour::new(0.1, 0.9, 0.2),
    );

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(origin, (position - origin).normalise());

            if let Some(list) = sphere.intersect(&ray) {
                if let Some(hit) = list.hit() {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(&point);
                    let eye = -ray.direction;

                    let colour = hit
                        .object
                        .material
                        .lighting(&light, &point, &eye, &normal);
                    canvas.write_pixel(x, y, &colour);
                }
            }
        }
    }

    write("image.ppm", canvas.to_ppm())
}
