use std::time::Instant;

use console::Term;
use indicatif::{
    HumanCount, HumanDuration, ProgressBar, ProgressFinish, ProgressIterator,
    ProgressStyle,
};

use crate::{
    math::{Angle, Point, Ray, Transformable, Transformation},
    Canvas, World,
};

/// `Camera` holds all the data representing our view into the scene.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    horizontal_size: usize,
    vertical_size: usize,
    field_of_view: Angle,
    inverse_transformation: Transformation,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    #[must_use]
    pub fn new(
        horizontal_size: usize,
        vertical_size: usize,
        field_of_view: Angle,
        transformation: Transformation,
    ) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        #[allow(clippy::cast_precision_loss)]
        let horizontal_float = horizontal_size as f64;
        #[allow(clippy::cast_precision_loss)]
        let aspect = horizontal_float / vertical_size as f64;

        let (half_width, half_height) = if aspect > 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Self {
            horizontal_size,
            vertical_size,
            field_of_view,
            inverse_transformation: transformation.invert(),
            half_width,
            half_height,
            pixel_size: half_width * 2.0 / horizontal_float,
        }
    }

    /// Renders the given `World` using the given camera.
    ///
    /// # Panics
    ///
    /// This function should not panic as all the values unwrapped should be
    /// valid but will if there is an error in the formatting for progress
    /// somewhere.
    #[must_use]
    pub fn render(&self, world: &World, depth: u32, quiet: bool) -> Canvas {
        if !quiet {
            println!(
                "Size {} by {}, field of view {:.1} degrees",
                HumanCount(self.horizontal_size.try_into().unwrap()),
                HumanCount(self.vertical_size.try_into().unwrap()),
                self.field_of_view.to_degrees()
            );

            println!("Rendering scene...");
        }

        let bar = if quiet {
            ProgressBar::hidden()
        } else {
            ProgressBar::new(self.horizontal_size.try_into().unwrap())
                .with_style(
                    ProgressStyle::with_template(
                        "\
{prefix} {bar:40.cyan/blue} {human_pos:>7}/{human_len:7} ({percent}%)
Elapsed: {elapsed}, estimated: {eta}, rows/sec: {per_sec}",
                    )
                    .unwrap()
                    .progress_chars("#>-"),
                )
                .with_prefix("Rows")
                .with_finish(ProgressFinish::AndClear)
        };

        let started = Instant::now();

        let mut canvas = Canvas::new(self.horizontal_size, self.vertical_size);

        for x in (0..self.horizontal_size).progress_with(bar) {
            for y in 0..self.vertical_size {
                let ray = self.ray_for_pixel(x, y);

                let colour = world.colour_at(&ray, depth);

                canvas.write_pixel(x, y, &colour);
            }
        }

        if !quiet {
            Term::stdout().clear_last_lines(1).unwrap();

            println!(
                "Rendering scene...done\nRendered {} rows in {}",
                HumanCount(self.horizontal_size.try_into().unwrap()),
                HumanDuration(started.elapsed())
            );
        }

        canvas
    }

    #[must_use]
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        #[allow(clippy::cast_precision_loss)]
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        #[allow(clippy::cast_precision_loss)]
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = Point::new(world_x, world_y, -1.0)
            .apply(&self.inverse_transformation);

        let origin = Point::origin().apply(&self.inverse_transformation);

        Ray::new(origin, (pixel - origin).normalise())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    use super::*;
    use crate::math::{float::*, Vector};

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn creating_a_camera() {
        let h = 160;
        let v = 120;
        let f = Angle(FRAC_PI_2);
        let t = Transformation::new();

        let c = Camera::new(h, v, f, t);

        assert_eq!(c.horizontal_size, h);
        assert_eq!(c.vertical_size, v);
        assert_approx_eq!(c.inverse_transformation, t);
        assert_approx_eq!(c.half_width, 1.0);
        assert_approx_eq!(c.half_height, 0.75);
        assert_approx_eq!(c.pixel_size, 0.012_5);

        let c = Camera::new(200, 125, f, t);
        assert_approx_eq!(c.half_width, 1.0);
        assert_approx_eq!(c.half_height, 0.625);
        assert_approx_eq!(c.pixel_size, 0.01);

        let c = Camera::new(125, 200, f, t);
        assert_approx_eq!(c.half_width, 0.625);
        assert_approx_eq!(c.half_height, 1.0);
        assert_approx_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_canvas() {
        let c = Camera::new(
            201,
            101,
            Angle::from_degrees(90.0),
            Transformation::new(),
        );

        assert_approx_eq!(
            c.ray_for_pixel(100, 50),
            Ray::new(Point::origin(), -Vector::z_axis())
        );

        assert_approx_eq!(
            c.ray_for_pixel(0, 0),
            Ray::new(
                Point::origin(),
                Vector::new(0.665_19, 0.332_59, -0.668_51)
            ),
            epsilon = 0.000_01
        );

        let mut c = c;
        c.inverse_transformation = Transformation::new()
            .translate(0.0, -2.0, 5.0)
            .rotate_y(Angle(FRAC_PI_4))
            .invert();

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            c.ray_for_pixel(100, 50),
            Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(sqrt_2_div_2, 0.0, -sqrt_2_div_2)
            )
        );
    }
}
