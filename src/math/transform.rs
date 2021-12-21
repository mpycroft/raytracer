use std::ops::Mul;

use super::Matrix;

/// The Transformable trait describes how to apply a Transform to any given
/// object, implementing this allows us to .apply() a Transform to an object via
/// this trait. This is really just some syntactic sugar so we always apply
/// Transform's to objects rather than transform objects with a given Transform.
pub trait Transformable<'a> {
    fn apply(&'a self, transform: &Transform) -> Self;
}

/// Blanket implementation of Transformable for objects that can be multiplied
/// by a matrix e.g. Points and Vectors.
impl<'a, T> Transformable<'a> for T
where
    Matrix<4>: Mul<T, Output = T>,
    T: 'a + Copy,
{
    fn apply(&'a self, transform: &Transform) -> Self {
        transform.data * *self
    }
}

/// A Transform is a wrapper around a 4 dimensional matrix allowing a more
/// ergonomic use of transformations. Transformations can be chained in an
/// obvious way e.g. Transform::new().rotate_x(2.3).scale(1.0, 0.5, 1.0) which
/// will perform the multiplications in reverse order as expected e.g. scale *
/// rotate_x.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Transform {
    data: Matrix<4>,
}

impl Transform {
    pub fn new() -> Self {
        Self::from_matrix(Matrix::identity())
    }

    fn from_matrix(data: Matrix<4>) -> Self {
        Self { data }
    }

    pub fn apply<'a, T: Transformable<'a>>(&self, object: &'a T) -> T {
        object.apply(self)
    }

    /// The invert function returns a new transformation rather than changing
    /// the internal data and allowing chaining as other functions do. Panics if
    /// the transformation cannot be inverted.
    pub fn invert(&self) -> Self {
        Self::from_matrix(
            self.data
                .invert()
                .expect("Transformation matrix could not be inverted"),
        )
    }

    /// The transpose function, much like invert, is not meant for chaining with
    /// other transforms but to produce a new Transform object that contains a
    /// transpose of the current object.
    pub fn transpose(&self) -> Self {
        Self::from_matrix(self.data.transpose())
    }

    pub fn rotate_x(&mut self, radians: f64) -> Self {
        self.data = Matrix::rotate_x(radians) * self.data;

        *self
    }

    pub fn rotate_y(&mut self, radians: f64) -> Self {
        self.data = Matrix::rotate_y(radians) * self.data;

        *self
    }

    pub fn rotate_z(&mut self, radians: f64) -> Self {
        self.data = Matrix::rotate_z(radians) * self.data;

        *self
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) -> Self {
        self.data = Matrix::scale(x, y, z) * self.data;

        *self
    }

    pub fn shear(
        &mut self,
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Self {
        self.data = Matrix::shear(xy, xz, yx, yz, zx, zy) * self.data;

        *self
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> Self {
        self.data = Matrix::translate(x, y, z) * self.data;

        *self
    }
}

add_approx_traits!(Transform { data });

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::*;

    use super::*;
    use crate::math::{Point, Vector};

    #[test]
    fn new() {
        assert_relative_eq!(Transform::new().data, Matrix::identity());
    }

    #[test]
    fn apply() {
        let p = Point::new(1.0, 2.0, 3.0);

        let mut t = Transform::new();

        assert_relative_eq!(t.apply(&p), p);

        assert_relative_eq!(
            t.scale(1.0, 1.0, 2.0).apply(&p),
            Point::new(1.0, 2.0, 6.0)
        );
    }

    #[test]
    fn invert() {
        let v = Vector::new(5.1, -2.3, 9.52);

        let t = Transform::new()
            .rotate_x(1.5)
            .scale(1.0, 2.0, 4.3)
            .translate(0.0, 1.0, 2.3)
            .rotate_y(1.0);

        assert_relative_eq!(t.invert().apply(&t.apply(&v)), v);
    }

    #[test]
    #[should_panic]
    fn invert_panic() {
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
    fn transpose() {
        assert_relative_eq!(
            Transform::new().translate(2.5, 3.1, -1.0).transpose().data,
            Matrix::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [2.5, 3.1, -1.0, 1.0]
            ])
        );
    }

    #[test]
    fn rotate_x() {
        assert_relative_eq!(
            Transform::new()
                .rotate_x(FRAC_PI_2)
                .apply(&Point::new(0.0, 1.0, 0.0)),
            Point::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn rotate_y() {
        assert_relative_eq!(
            Transform::new()
                .rotate_y(FRAC_PI_2)
                .apply(&Point::new(0.0, 0.0, 1.0)),
            Point::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn rotate_z() {
        assert_relative_eq!(
            Transform::new()
                .rotate_z(FRAC_PI_2)
                .apply(&Point::new(1.0, 0.0, 0.0)),
            Point::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn scale() {
        assert_relative_eq!(
            Transform::new()
                .scale(1.0, 0.5, 2.0)
                .apply(&Vector::new(1.6, 3.0, 4.3)),
            Vector::new(1.6, 1.5, 8.6)
        );
    }

    #[test]
    fn shear() {
        assert_relative_eq!(
            Transform::new()
                .shear(1.0, 2.0, 0.0, 1.0, 1.0, 0.0)
                .apply(&Point::new(1.0, 1.0, 1.0)),
            Point::new(4.0, 2.0, 2.0)
        );
    }

    #[test]
    fn translate() {
        assert_relative_eq!(
            Transform::new()
                .translate(1.5, 2.3, 7.5)
                .apply(&Point::new(3.1, 5.5, 2.13)),
            Point::new(4.6, 7.8, 9.63)
        );
    }

    #[test]
    fn chaining_transforms() {
        assert_relative_eq!(
            Transform::new()
                .rotate_y(FRAC_PI_2)
                .translate(1.0, 1.0, 1.0)
                .scale(2.5, 2.5, 2.5)
                .translate(-2.0, 3.0, 9.5)
                .apply(&Point::new(0.0, 0.0, 1.0)),
            Point::new(3.0, 5.5, 12.0)
        );
    }

    #[test]
    fn approx() {
        let t1 = Transform::new().translate(1.0, 2.5, 0.9);
        let t2 = Transform::new().translate(1.0, 2.5, 0.9);
        let t3 = Transform::new().rotate_x(1.8);

        assert_abs_diff_eq!(t1, t2);
        assert_abs_diff_ne!(t1, t3);

        assert_relative_eq!(t1, t2);
        assert_relative_ne!(t1, t3);

        assert_ulps_eq!(t1, t2);
        assert_ulps_ne!(t1, t3);
    }
}
