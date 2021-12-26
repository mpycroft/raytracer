use crate::{
    math::{Point, Ray, Transform},
    world::World,
    Canvas,
};

/// The Camera struct holds the data representing our camera view into the
/// scene.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera {
    pub horizontal: usize,
    pub vertical: usize,
    pub field_of_view: f64,
    pub transform: Transform,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(
        horizontal: usize,
        vertical: usize,
        field_of_view: f64,
        transform: Transform,
    ) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = horizontal as f64 / vertical as f64;

        let (half_width, half_height) = if aspect > 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Camera {
            horizontal,
            vertical,
            field_of_view,
            transform,
            pixel_size: half_width * 2.0 / horizontal as f64,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse = self.transform.invert();
        let pixel = inverse.apply(&Point::new(world_x, world_y, -1.0));
        let origin = inverse.apply(&Point::origin());
        let direction = (pixel - origin).normalise();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.horizontal, self.vertical);

        for y in 0..(self.vertical - 1) {
            for x in 0..(self.horizontal - 1) {
                let ray = self.ray_for_pixel(x, y);
                let colour = world.colour_at(&ray);

                image.write_pixel(x, y, colour);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    use approx::*;

    use super::*;
    use crate::{
        math::{Angle, Vector},
        Colour,
    };

    #[test]
    fn new() {
        let c = Camera::new(160, 120, FRAC_PI_2, Transform::new());

        assert_eq!(c.horizontal, 160);
        assert_eq!(c.vertical, 120);
        assert_float_relative_eq!(c.field_of_view, FRAC_PI_2);
        assert_relative_eq!(c.transform, Transform::new());
        assert_float_relative_eq!(c.pixel_size, 0.012_5);

        let c = Camera::new(200, 125, FRAC_PI_2, Transform::new());
        assert_float_relative_eq!(c.pixel_size, 0.01);

        let c = Camera::new(125, 200, FRAC_PI_2, Transform::new());
        assert_float_relative_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn ray_for_pixel() {
        let mut c = Camera::new(201, 101, FRAC_PI_2, Transform::new());

        assert_relative_eq!(
            c.ray_for_pixel(100, 50),
            Ray::new(Point::origin(), -Vector::z_axis())
        );

        assert_relative_eq!(
            c.ray_for_pixel(0, 0),
            Ray::new(
                Point::origin(),
                Vector::new(0.665_186, 0.332_593, -0.668_512)
            )
        );

        c.transform = Transform::from_translate(0.0, -2.0, 5.0)
            .rotate_y(Angle::from_radians(FRAC_PI_4));

        assert_relative_eq!(
            c.ray_for_pixel(100, 50),
            Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0)
            )
        );
    }

    #[test]
    fn render() {
        let w = World::default();
        let c = Camera::new(
            11,
            11,
            FRAC_PI_2,
            Transform::view_transform(
                &Point::new(0.0, 0.0, -5.0),
                &Point::origin(),
                &Vector::y_axis(),
            ),
        );

        let i = c.render(&w);

        assert_relative_eq!(
            i.get_pixel(5, 5),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );
    }
}
