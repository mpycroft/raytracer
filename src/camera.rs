use crate::math::Transformation;

/// `Camera` holds all the data representing our view into the scene.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    horizontal_size: usize,
    vertical_size: usize,
    field_of_view: f64,
    transformation: Transformation,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    #[must_use]
    pub fn new(
        horizontal_size: usize,
        vertical_size: usize,
        field_of_view: f64,
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

        Camera {
            horizontal_size,
            vertical_size,
            field_of_view,
            transformation,
            half_width,
            half_height,
            pixel_size: half_width * 2.0 / horizontal_float,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::math::float::assert_approx_eq;

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn creating_a_camera() {
        let h = 160;
        let v = 120;
        let f = PI / 2.0;
        let t = Transformation::new();

        let c = Camera::new(h, v, f, t);

        assert_eq!(c.horizontal_size, h);
        assert_eq!(c.vertical_size, v);
        assert_approx_eq!(c.field_of_view, f);
        assert_approx_eq!(c.transformation, t);
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
}
