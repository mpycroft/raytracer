use crate::{
    math::{Point, Ray, Transform},
    util::float::Float,
    world::World,
    Canvas,
};

/// The Camera struct holds the data representing our camera view into the
/// scene.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera<T: Float> {
    pub horizontal: usize,
    pub vertical: usize,
    pub field_of_view: T,
    pub transform: Transform<T>,
    pub pixel_size: T,
    pub half_width: T,
    pub half_height: T,
}

impl<T: Float> Camera<T> {
    pub fn new(
        horizontal: usize,
        vertical: usize,
        field_of_view: T,
        transform: Transform<T>,
    ) -> Self {
        let half_view = (field_of_view / T::from(2.0).unwrap()).tan();
        let aspect = T::from(horizontal).unwrap() / T::from(vertical).unwrap();

        let (half_width, half_height) = if aspect > T::one() {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Camera {
            horizontal,
            vertical,
            field_of_view,
            transform,
            pixel_size: half_width * T::from(2.0f64).unwrap()
                / T::from(horizontal).unwrap(),
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray<T> {
        let x_offset =
            (T::from(x).unwrap() + T::from(0.5f64).unwrap()) * self.pixel_size;
        let y_offset =
            (T::from(y).unwrap() + T::from(0.5f64).unwrap()) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse = self.transform.invert();
        let pixel = inverse.apply(&Point::new(world_x, world_y, -T::one()));
        let origin = inverse.apply(&Point::origin());
        let direction = (pixel - origin).normalise();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World<T>) -> Canvas<T> {
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
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, FRAC_PI_2, Transform::new());

        assert_eq!(c.horizontal, 160);
        assert_eq!(c.vertical, 120);
        assert_float_relative_eq!(c.field_of_view, FRAC_PI_2);
        assert_relative_eq!(c.transform, Transform::new());
        assert_float_relative_eq!(c.pixel_size, 0.012_5);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        assert_float_relative_eq!(
            Camera::new(200, 125, FRAC_PI_2, Transform::new()).pixel_size,
            0.01
        );
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        assert_float_relative_eq!(
            Camera::new(125, 200, FRAC_PI_2, Transform::new()).pixel_size,
            0.01
        );
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        assert_relative_eq!(
            Camera::new(201, 101, FRAC_PI_2, Transform::new())
                .ray_for_pixel(100, 50),
            Ray::new(Point::origin(), -Vector::z_axis())
        );
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        assert_relative_eq!(
            Camera::new(201, 101, FRAC_PI_2, Transform::new())
                .ray_for_pixel(0, 0),
            Ray::new(
                Point::origin(),
                Vector::new(0.665_186, 0.332_593, -0.668_512)
            )
        );
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        assert_relative_eq!(
            Camera::new(
                201,
                101,
                FRAC_PI_2,
                Transform::from_translate(0.0, -2.0, 5.0)
                    .rotate_y(Angle::from_radians(FRAC_PI_4)),
            )
            .ray_for_pixel(100, 50),
            Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0)
            )
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        assert_relative_eq!(
            Camera::new(
                11,
                11,
                FRAC_PI_2,
                Transform::view_transform(
                    &Point::new(0.0, 0.0, -5.0),
                    &Point::origin(),
                    &Vector::y_axis(),
                ),
            )
            .render(&World::default())
            .get_pixel(5, 5),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );
    }
}
