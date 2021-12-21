use std::fs::write;

use raytracer::{
    math::{Point, Ray, Transform},
    Canvas, Colour, Intersectable, Material, PointLight, Sphere,
};

fn main() {
    let canvas_pixels = 250;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let origin = Point::new(0.0, 0.0, -5.0);

    let wall_size = 7.0;
    let wall_z = 10.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let sphere = Sphere::new(
        Transform::from_scale(0.5, 1.0, 1.0)
            .rotate_z(0.7)
            .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Material::new(Colour::new(1.0, 0.2, 1.0), 0.1, 0.9, 0.9, 200.0),
    );

    let light = PointLight::new(Colour::white(), Point::new(10.0, 10.0, -10.0));

    for y in 0..(canvas_pixels - 1) {
        let world_y = half - pixel_size * y as f64;

        for x in 0..(canvas_pixels - 1) {
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

                    canvas.write_pixel(x, y, colour);
                }
            }
        }
    }

    write("image.ppm", canvas.to_ppm()).unwrap();
}
