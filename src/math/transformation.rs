use std::ops::Mul;

use float_cmp::{ApproxEq, F64Margin};

use super::matrix::Matrix;

/// A `Transformation` is a wrapper around a 4 dimensional matrix allowing a
/// more ergonomic use of transformations. Transformations can be chained in an
/// obvious way e.g. `Transformation::new().rotate_x(2.3).scale(1.0, 0.5, 1.0)`
/// which will perform the multiplications in reverse order as expected e.g.
/// `scale` * `rotate_x`.
#[derive(Clone, Copy, Debug)]
pub struct Transformation(pub Matrix<4>);

impl Transformation {
    #[must_use]
    pub fn new() -> Self {
        Self(Matrix::identity())
    }

    #[must_use]
    pub fn apply<T: Copy>(&self, object: &T) -> <Matrix<4> as Mul<T>>::Output
    where
        Matrix<4>: Mul<T>,
    {
        self.0 * *object
    }

    /// Unlike the other function on `Transform`, `invert` is not intended for
    /// chaining, instead it returns a new `Transform` with the inverted matrix.
    ///
    /// # Panics
    ///
    /// In general the simple transformation matrices we use should always be
    /// invertible, therefore we just panic if we are unable to invert.
    #[must_use]
    pub fn invert(&self) -> Self {
        Self(self.0.invert().unwrap_or_else(|err| panic!("{err}")))
    }

    /// Like the `invert` function `transpose` does not chain like other
    /// functions, instead it returns a new `Transform` with the transposed
    /// matrix.
    #[must_use]
    pub fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    #[must_use]
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.0 = Matrix::translate(x, y, z) * self.0;

        self
    }

    #[must_use]
    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.0 = Matrix::scale(x, y, z) * self.0;

        self
    }

    #[must_use]
    pub fn rotate_x(mut self, radians: f64) -> Self {
        self.0 = Matrix::rotate_x(radians) * self.0;

        self
    }

    #[must_use]
    pub fn rotate_y(mut self, radians: f64) -> Self {
        self.0 = Matrix::rotate_y(radians) * self.0;

        self
    }

    #[must_use]
    pub fn rotate_z(mut self, radians: f64) -> Self {
        self.0 = Matrix::rotate_z(radians) * self.0;

        self
    }

    #[must_use]
    pub fn shear(
        mut self,
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Self {
        self.0 = Matrix::shear(xy, xz, yx, yz, zx, zy) * self.0;

        self
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::new()
    }
}

impl ApproxEq for Transformation {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.0.approx_eq(other.0, margin)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};

    use super::*;
    use crate::math::{float::*, Point, Vector};

    #[test]
    fn creating_a_transformation() {
        let t = Transformation::new();

        assert_approx_eq!(t.0, Matrix::<4>::identity());

        assert_approx_eq!(t, Transformation::default());
    }

    #[test]
    fn ordering_of_transformations() {
        let p = Point::new(3.0, 2.0, 1.0);
        let o = Point::new(8.0, 8.0, 8.0);

        let t =
            Transformation::new().translate(1.0, 2.0, 3.0).scale(2.0, 2.0, 2.0);

        assert_approx_eq!(t.apply(&p), o);

        assert_approx_eq!(
            Transformation::new()
                .translate(1.0, 2.0, 3.0)
                .scale(2.0, 2.0, 2.0)
                .apply(&p),
            o
        );

        let mut t = Transformation::new();
        t = t.translate(1.0, 2.0, 3.0).scale(2.0, 2.0, 2.0);

        assert_approx_eq!(t.apply(&p), o);

        let mut t = Transformation::new();
        t = t.translate(1.0, 2.0, 3.0);
        t = t.scale(2.0, 2.0, 2.0);

        assert_approx_eq!(t.apply(&p), o);
    }

    #[test]
    fn applying_a_transformation() {
        let p = Point::new(1.5, 2.5, 3.5);

        let t = Transformation::new();
        assert_approx_eq!(t.apply(&p), p);

        assert_approx_eq!(
            t.scale(2.0, 2.0, 2.0).apply(&p),
            Point::new(3.0, 5.0, 7.0)
        );
    }

    #[test]
    fn chaining_multiple_transformations() {
        assert_approx_eq!(
            Transformation::new()
                .rotate_y(FRAC_PI_2)
                .translate(1.0, 1.0, 1.0)
                .scale(2.5, 2.5, 2.5)
                .translate(-2.0, 3.0, 9.5)
                .apply(&Point::new(0.0, 0.0, 1.0)),
            Point::new(3.0, 5.5, 12.0)
        );
    }

    #[test]
    fn inverting_a_transform() {
        let v = Vector::new(5.1, -2.3, 9.52);

        let t = Transformation::new()
            .rotate_x(1.5)
            .scale(1.0, 2.0, 4.3)
            .translate(0.0, 1.0, 2.3)
            .rotate_y(1.0);

        assert_approx_eq!(t.invert().apply(&t.apply(&v)), v);
    }

    #[test]
    #[should_panic(expected = "\
Tried to invert a Matrix that cannot be inverted - Matrix<4>([
    [12.0, 1.0, 2.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [-2.0, 0.0, 1.0, 0.0],
    [-1.5, 9.3, 0.0, 2.0],
])")]
    fn inverting_a_non_invertible_transform() {
        let _ = Transformation(Matrix([
            [12.0, 1.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [-2.0, 0.0, 1.0, 0.0],
            [-1.5, 9.3, 0.0, 2.0],
        ]))
        .invert();
    }

    #[test]
    fn transposing_a_transform() {
        assert_approx_eq!(
            Transformation::new().translate(2.5, 3.1, -1.0).transpose().0,
            Matrix::translate(2.5, 3.1, -1.0).transpose()
        );
    }

    #[test]
    fn translating_a_transformation() {
        assert_approx_eq!(
            Transformation::new().translate(1.0, 2.0, 3.0).0,
            Matrix::translate(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn scaling_a_transformation() {
        assert_approx_eq!(
            Transformation::new().scale(2.0, 2.0, 2.5).0,
            Matrix::scale(2.0, 2.0, 2.5)
        );
    }

    #[test]
    fn rotating_a_transformation() {
        assert_approx_eq!(
            Transformation::new().rotate_x(FRAC_PI_2).0,
            Matrix::rotate_x(FRAC_PI_2)
        );

        assert_approx_eq!(
            Transformation::new().rotate_y(FRAC_PI_6).0,
            Matrix::rotate_y(FRAC_PI_6)
        );

        assert_approx_eq!(
            Transformation::new().rotate_z(PI).0,
            Matrix::rotate_z(PI)
        );
    }

    #[test]
    fn shearing_a_transformation() {
        assert_approx_eq!(
            Transformation::new().shear(1.0, 0.5, 2.0, 1.0, 0.0, 0.9).0,
            Matrix::shear(1.0, 0.5, 2.0, 1.0, 0.0, 0.9)
        );
    }

    #[test]
    fn comparing_transformations() {
        let t1 = Transformation(Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
        ]));
        let t2 = Transformation(Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
        ]));
        let t3 = Transformation(Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.000_001],
        ]));

        assert_approx_eq!(t1, t2);

        assert_approx_ne!(t1, t3);
    }
}
