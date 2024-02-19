use std::{io::Write, iter::from_fn, time::Instant};

use anyhow::Result;
use indicatif::{
    HumanCount, HumanDuration, ParallelProgressIterator, ProgressBar,
    ProgressDrawTarget, ProgressFinish, ProgressIterator, ProgressStyle,
};
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;
use serde::{Deserialize, Deserializer};

use crate::{
    math::{
        float::impl_approx_eq, Angle, Point, Ray, Transformable,
        Transformation, Vector,
    },
    Canvas, Colour, Output, World,
};

/// `Camera` holds all the data representing our view into the scene.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    horizontal_size: u32,
    vertical_size: u32,
    field_of_view: Angle,
    inverse_transformation: Transformation,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    #[must_use]
    pub fn new(
        horizontal_size: u32,
        vertical_size: u32,
        field_of_view: Angle,
        transformation: Transformation,
    ) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        #[allow(clippy::cast_precision_loss)]
        let horizontal_float = f64::from(horizontal_size);
        #[allow(clippy::cast_precision_loss)]
        let aspect = horizontal_float / f64::from(vertical_size);

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

    #[must_use]
    pub const fn horizontal_size(&self) -> u32 {
        self.horizontal_size
    }

    #[must_use]
    pub const fn vertical_size(&self) -> u32 {
        self.vertical_size
    }

    /// Renders the given `World` using the given camera.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't convert values or there
    /// is an error writing output.
    pub fn render<O: Write, R: Rng>(
        &self,
        world: &World,
        depth: u32,
        single_threaded: bool,
        output: &mut Output<O>,
        rng: &mut R,
    ) -> Result<Canvas> {
        writeln!(
            output,
            "Size {} by {}, field of view {:.1} degrees",
            HumanCount(self.horizontal_size.into()),
            HumanCount(self.vertical_size.into()),
            self.field_of_view.to_degrees()
        )?;

        writeln!(output, "Rendering scene...")?;

        let bar = ProgressBar::new(self.vertical_size.into())
            .with_style(
                ProgressStyle::with_template(
                    "\
{prefix} {bar:40.cyan/blue} {human_pos:>7}/{human_len:7} ({percent}%)
Elapsed: {elapsed}, remaining: {eta}, rows/sec: {per_sec}",
                )?
                .progress_chars("#>-"),
            )
            .with_prefix("Rows")
            .with_finish(ProgressFinish::AndClear);

        bar.set_draw_target(if output.is_sink() {
            ProgressDrawTarget::hidden()
        } else {
            ProgressDrawTarget::stdout()
        });

        let started = Instant::now();

        let iterator_fn = |(y, seed)| {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

            let mut colours = Vec::with_capacity(self.vertical_size as usize);

            for x in 0..self.horizontal_size {
                let ray = self.ray_for_pixel(x, y);

                let colour = world.colour_at(&ray, depth, &mut rng);

                colours.push(colour);
            }

            colours
        };

        let seeds: Vec<u64> = from_fn(|| Some(rng.gen()))
            .take(self.vertical_size as usize)
            .collect();

        // Either does not appear to play nicely with rayon / std iterators so
        // there appears no nice way to simplify this check despite it looking
        // like it should be trivial to do so.
        let pixels: Vec<Colour> = if single_threaded {
            (0..self.vertical_size)
                .zip(seeds)
                .progress_with(bar)
                .flat_map(iterator_fn)
                .collect()
        } else {
            (0..self.vertical_size)
                .into_par_iter()
                .zip(seeds)
                .progress_with(bar)
                .flat_map(iterator_fn)
                .collect()
        };

        output.clear_last_line()?;

        writeln!(
            output,
            "Rendering scene...done\nRendered {} rows in {}",
            HumanCount(self.horizontal_size.into()),
            HumanDuration(started.elapsed())
        )?;

        Ok(Canvas::with_vec(self.horizontal_size, self.vertical_size, pixels))
    }

    #[must_use]
    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        #[allow(clippy::cast_precision_loss)]
        let x_offset = (f64::from(x) + 0.5) * self.pixel_size;
        #[allow(clippy::cast_precision_loss)]
        let y_offset = (f64::from(y) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = Point::new(world_x, world_y, -1.0)
            .apply(&self.inverse_transformation);

        let origin = Point::origin().apply(&self.inverse_transformation);

        Ray::new(origin, (pixel - origin).normalise())
    }
}

impl_approx_eq!(Camera {
    eq horizontal_size,
    eq vertical_size,
    field_of_view,
    half_width,
    half_height,
    pixel_size
});

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Camera {
            pub width: u32,
            pub height: u32,
            pub field_of_view: Angle,
            pub from: Point,
            pub to: Point,
            pub up: Vector,
        }

        let camera = Camera::deserialize(deserializer)?;

        Ok(Self::new(
            camera.width,
            camera.height,
            camera.field_of_view,
            Transformation::view_transformation(
                camera.from,
                camera.to,
                camera.up,
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI, SQRT_2};

    use serde_yaml::from_str;

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
    fn get_size_of_camera() {
        let c = Camera::new(20, 30, Angle(PI), Transformation::new());

        assert_eq!(c.horizontal_size(), 20);
        assert_eq!(c.vertical_size(), 30);
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

    #[test]
    fn comparing_cameras() {
        let c1 = Camera::new(
            100,
            80,
            Angle(FRAC_PI_2),
            Transformation::view_transformation(
                Point::new(1.0, 2.0, 3.0),
                Point::origin(),
                Vector::y_axis(),
            ),
        );
        let c2 = Camera::new(
            100,
            80,
            Angle(FRAC_PI_2),
            Transformation::view_transformation(
                Point::new(1.0, 2.0, 3.0),
                Point::origin(),
                Vector::y_axis(),
            ),
        );
        let c3 = Camera::new(
            100,
            80,
            Angle(FRAC_PI_4),
            Transformation::view_transformation(
                Point::new(1.0, 2.0, 3.0),
                Point::origin(),
                Vector::y_axis(),
            ),
        );

        assert_approx_eq!(c1, c2);

        assert_approx_ne!(c1, c3);
    }

    #[test]
    fn deserialize_camera() {
        let c: Camera = from_str(
            "\
width: 200
height: 150
field-of-view: \"PI / 3\"
from: [1, 2, 3]
to: [0, 1.5, 0.0]
up: [0, 1, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            c,
            Camera::new(
                200,
                150,
                Angle(FRAC_PI_3),
                Transformation::view_transformation(
                    Point::new(1.0, 2.0, 3.0),
                    Point::new(0.0, 1.5, 0.0),
                    Vector::y_axis()
                )
            )
        );
    }
}
