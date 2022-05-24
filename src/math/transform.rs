use std::ops::Mul;

use paste::paste;

use super::{Angle, Matrix, Point, Vector};
use crate::util::float::Float;

/// The `Transformable` trait describes how to apply a `Transform` to any given
/// object, implementing this allows us to .apply() a `Transform` to an object
/// via this trait. This is really just some syntactic sugar so we always apply
/// Transform's to objects rather than transform objects with a given Transform.
pub trait Transformable<'a, T: Float> {
    fn apply(&'a self, transform: &Transform<T>) -> Self;
}

/// Blanket implementation of Transformable for objects that can be multiplied
/// by a matrix e.g. Points and Vectors.
impl<'a, T: Float, U> Transformable<'a, T> for U
where
    Matrix<T, 4>: Mul<U, Output = U>,
    U: 'a + Copy,
{
    fn apply(&'a self, transform: &Transform<T>) -> Self {
        transform.data * *self
    }
}

/// A `Transform` is a wrapper around a 4 dimensional matrix allowing a more
/// ergonomic use of transformations. Transformations can be chained in an
/// obvious way e.g. `Transform::new().rotate_x(2.3).scale(1.0, 0.5, 1.0)` which
/// will perform the multiplications in reverse order as expected e.g. scale *
/// `rotate_x`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Transform<T: Float> {
    data: Matrix<T, 4>,
}

/// This macro adds a function to create a new transform directly (prefixed with
/// from_) and to perform the transform on an already existing Transform object.
macro_rules! add_transform_fns {
    ($name:ident($($arg:ident: $type:ty),+ $(,)?)) => {
        paste! {
            pub fn [<from_ $name>]($($arg: $type),+) -> Self {
                Self::from_matrix(Matrix::$name($($arg),+))
            }

            pub fn $name(&mut self, $($arg: $type),+) -> Self {
                self.data = Matrix::$name($($arg),+) * self.data;

                *self
            }
        }
    };
}

impl<T: Float> Transform<T> {
    pub fn new() -> Self {
        Self::from_matrix(Matrix::identity())
    }

    pub fn view_transform(
        from: &Point<T>,
        to: &Point<T>,
        up: &Vector<T>,
    ) -> Self {
        Self::from_matrix(Matrix::view_transform(from, to, up))
    }

    fn from_matrix(data: Matrix<T, 4>) -> Self {
        Self { data }
    }

    pub fn apply<'a, U: Transformable<'a, T>>(&self, object: &'a U) -> U {
        object.apply(self)
    }

    /// The `invert` function returns a new transformation rather than changing
    /// the internal data and allowing chaining as other functions do. Panics if
    /// the transformation cannot be inverted.
    pub fn invert(&self) -> Self {
        Self::from_matrix(
            self.data
                .invert()
                .expect("Transformation matrix could not be inverted"),
        )
    }

    /// The `transpose` function, much like invert, is not meant for chaining
    /// with other transforms but to produce a new Transform object that
    /// contains a transpose of the current object.
    pub fn transpose(&self) -> Self {
        Self::from_matrix(self.data.transpose())
    }

    add_transform_fns!(rotate_x(angle: Angle<T>));
    add_transform_fns!(rotate_y(angle: Angle<T>));
    add_transform_fns!(rotate_z(angle: Angle<T>));

    add_transform_fns!(scale(x: T, y: T, z: T));

    add_transform_fns!(shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T,));

    add_transform_fns!(translate(x: T, y: T, z: T));
}

impl<T: Float> Default for Transform<T> {
    fn default() -> Self {
        Self::new()
    }
}

add_approx_traits!(Transform<T> { data });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

    use approx::*;

    use super::*;

    #[test]
    fn a_new_transform_is_the_identity_transformation() {
        assert_relative_eq!(Transform::<f64>::new().data, Matrix::identity());
    }

    #[test]
    fn a_default_transform_is_the_identity_transformation() {
        assert_relative_eq!(Transform::<f64>::new(), Transform::default());
    }

    #[test]
    fn creating_a_transform_from_a_view_transformation() {
        let from = Point::new(2.0, 3.0, 4.0);
        let to = Point::new(-1.0, 0.0, 5.0);
        let up = Vector::new(1.0, 0.3, -0.3);

        assert_relative_eq!(
            Transform::view_transform(&from, &to, &up).data,
            Matrix::view_transform(&from, &to, &up)
        );
    }

    #[test]
    fn applying_a_transform_to_a_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_relative_eq!(Transform::new().apply(&p), p);

        assert_relative_eq!(
            Transform::from_scale(1.0, 1.0, 2.0).apply(&p),
            Point::new(1.0, 2.0, 6.0)
        );
    }

    #[test]
    fn inverting_a_transform() {
        let v = Vector::new(5.1, -2.3, 9.52);

        let t = Transform::from_rotate_x(Angle::from_radians(1.5))
            .scale(1.0, 2.0, 4.3)
            .translate(0.0, 1.0, 2.3)
            .rotate_y(Angle::from_radians(1.0));

        assert_relative_eq!(t.invert().apply(&t.apply(&v)), v);
    }

    #[test]
    #[should_panic]
    fn inverting_a_non_invertible_transform_panics() {
        let _ = Transform::from_matrix(Matrix::new([
            [12.0, 1.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [-2.0, 0.0, 1.0, 0.0],
            [-1.5, 9.3, 0.0, 2.0],
        ]))
        .invert()
        .apply(&Point::origin());
    }

    #[test]
    fn transposing_a_transform() {
        assert_relative_eq!(
            Transform::from_translate(2.5, 3.1, -1.0).transpose().data,
            Matrix::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [2.5, 3.1, -1.0, 1.0]
            ])
        );
    }

    #[test]
    fn creating_a_transform_from_a_x_rotation() {
        assert_relative_eq!(
            Transform::from_rotate_x(Angle::from_radians(0.95)).data,
            Matrix::rotate_x(Angle::from_radians(0.95))
        );
    }

    #[test]
    fn rotating_a_transform_around_the_x_axis() {
        assert_relative_eq!(
            Transform::new()
                .rotate_x(Angle::from_degrees(90.0))
                .apply(&Point::new(0.0, 1.0, 0.0)),
            Point::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn creating_a_transform_from_a_y_rotation() {
        assert_relative_eq!(
            Transform::from_rotate_y(Angle::from_radians(FRAC_PI_3)).data,
            Matrix::rotate_y(Angle::from_degrees(60.0))
        );
    }

    #[test]
    fn rotating_a_transform_around_the_y_axis() {
        assert_relative_eq!(
            Transform::new()
                .rotate_y(Angle::from_radians(FRAC_PI_2))
                .apply(&Point::new(0.0, 0.0, 1.0)),
            Point::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn creating_a_transform_from_a_z_rotation() {
        assert_relative_eq!(
            Transform::from_rotate_z(Angle::from_radians(2.15)).data,
            Matrix::rotate_z(Angle::from_radians(2.15))
        );
    }

    #[test]
    fn rotating_a_transform_around_the_z_axis() {
        assert_relative_eq!(
            Transform::new()
                .rotate_z(Angle::from_radians(FRAC_PI_2))
                .apply(&Point::new(1.0, 0.0, 0.0)),
            Point::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn creating_a_transform_from_a_scale() {
        assert_relative_eq!(
            Transform::from_scale(1.0, 2.5, 0.5).data,
            Matrix::scale(1.0, 2.5, 0.5)
        );
    }

    #[test]
    fn scaling_a_transform() {
        assert_relative_eq!(
            Transform::new()
                .scale(1.0, 0.5, 2.0)
                .apply(&Vector::new(1.6, 3.0, 4.3)),
            Vector::new(1.6, 1.5, 8.6)
        );
    }

    #[test]
    fn creating_a_transform_from_a_shear() {
        assert_relative_eq!(
            Transform::from_shear(1.0, 2.0, 1.0, 3.0, 2.0, 0.5).data,
            Matrix::shear(1.0, 2.0, 1.0, 3.0, 2.0, 0.5)
        );
    }

    #[test]
    fn shearing_a_transform() {
        assert_relative_eq!(
            Transform::new()
                .shear(1.0, 2.0, 0.0, 1.0, 1.0, 0.0)
                .apply(&Point::new(1.0, 1.0, 1.0)),
            Point::new(4.0, 2.0, 2.0)
        );
    }

    #[test]
    fn creating_a_transform_from_a_translation() {
        assert_relative_eq!(
            Transform::from_translate(0.0, -1.5, 2.0).data,
            Matrix::translate(0.0, -1.5, 2.0)
        );
    }

    #[test]
    fn translating_a_transform() {
        assert_relative_eq!(
            Transform::new()
                .translate(1.5, 2.3, 7.5)
                .apply(&Point::new(3.1, 5.5, 2.13)),
            Point::new(4.6, 7.8, 9.63)
        );
    }

    #[test]
    fn transform_functions_can_be_chained_together() {
        assert_relative_eq!(
            Transform::from_rotate_y(Angle::from_radians(FRAC_PI_2))
                .translate(1.0, 1.0, 1.0)
                .scale(2.5, 2.5, 2.5)
                .translate(-2.0, 3.0, 9.5)
                .apply(&Point::new(0.0, 0.0, 1.0)),
            Point::new(3.0, 5.5, 12.0)
        );
    }

    #[test]
    fn transforms_are_approximately_equal() {
        let t1 = Transform::from_translate(1.0, 2.5, 0.9);
        let t2 = Transform::from_translate(1.0, 2.5, 0.9);
        let t3 = Transform::from_rotate_x(Angle::from_radians(1.8));

        assert_abs_diff_eq!(t1, t2);
        assert_abs_diff_ne!(t1, t3);

        assert_relative_eq!(t1, t2);
        assert_relative_ne!(t1, t3);

        assert_ulps_eq!(t1, t2);
        assert_ulps_ne!(t1, t3);
    }
}
