use crate::math::Transform;

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
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::*;

    use super::*;

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
}
