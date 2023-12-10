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
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_transformation() {
        let t = Transformation::new();

        assert_approx_eq!(t.0, Matrix::<4>::identity());
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
