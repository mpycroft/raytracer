use derive_more::Index;
use float_cmp::{ApproxEq, F64Margin};

/// A Matrix is a square matrix of size N, stored in row major order.
#[derive(Clone, Copy, Debug, Index)]
pub struct Matrix<const N: usize>(pub [[f64; N]; N]);

impl<const N: usize> ApproxEq for Matrix<N> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        for (lhs_row, rhs_row) in self.0.iter().zip(other.0.iter()) {
            for (lhs, rhs) in lhs_row.iter().zip(rhs_row.iter()) {
                if !lhs.approx_eq(*rhs, margin) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_approx_eq!(m[0][0], 1.0);
        assert_approx_eq!(m[0][1], 2.0);
        assert_approx_eq!(m[0][2], 3.0);
        assert_approx_eq!(m[0][3], 4.0);

        assert_approx_eq!(m[1][0], 5.5);
        assert_approx_eq!(m[1][1], 6.5);
        assert_approx_eq!(m[1][2], 7.5);
        assert_approx_eq!(m[1][3], 8.5);

        assert_approx_eq!(m[2][0], 9.0);
        assert_approx_eq!(m[2][1], 10.0);
        assert_approx_eq!(m[2][2], 11.0);
        assert_approx_eq!(m[2][3], 12.0);

        assert_approx_eq!(m[3][0], 13.5);
        assert_approx_eq!(m[3][1], 14.5);
        assert_approx_eq!(m[3][2], 15.5);
        assert_approx_eq!(m[3][3], 16.5);

        let m = Matrix([[-3.0, 5.0], [-1.0, -2.0]]);

        assert_approx_eq!(m[0][0], -3.0);
        assert_approx_eq!(m[0][1], 5.0);
        assert_approx_eq!(m[1][0], -1.0);
        assert_approx_eq!(m[1][1], -2.0);

        let m = Matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_approx_eq!(m[0][0], -3.0);
        assert_approx_eq!(m[0][1], 5.0);
        assert_approx_eq!(m[0][2], 0.0);

        assert_approx_eq!(m[1][0], 1.0);
        assert_approx_eq!(m[1][1], -2.0);
        assert_approx_eq!(m[1][2], -7.0);

        assert_approx_eq!(m[2][0], 0.0);
        assert_approx_eq!(m[2][1], 1.0);
        assert_approx_eq!(m[2][2], 1.0);
    }

    #[test]
    fn indexing_into_a_matrix() {
        let m = Matrix([
            [1.1, 1.2, 1.3, 1.4],
            [5.1, 2.1, -5.6, 0.0],
            [0.0, 1.0, 2.0, -0.7],
            [0.0, 0.0, -0.0, 0.5],
        ]);

        let row = m[1];
        assert_approx_eq!(row[0], 5.1);
        assert_approx_eq!(row[1], 2.1);
        assert_approx_eq!(row[2], -5.6);
        assert_approx_eq!(row[3], 0.0);

        assert_approx_eq!(m[0][0], 1.1);
        assert_approx_eq!(m[1][2], -5.6);
        assert_approx_eq!(m[0][3], 1.4);
        assert_approx_eq!(m[3][1], 0.0);
    }

    #[test]
    #[should_panic]
    fn indexing_with_invalid_values() {
        let m = Matrix([
            [0.0, 0.0, 1.0, 0.0],
            [0.3, -2.0, -1.5, 0.0],
            [0.0, 0.0, 0.0, 0.2],
            [0.0, 1.0, 0.0, 0.3],
        ]);

        let _ = m[4][2];
    }

    #[test]
    fn comparing_matrices() {
        let m1 = Matrix([
            [2.1, 3.1, 4.6, 0.9],
            [-1.0, 0.0, -2.4, 7.1],
            [1_261.96, 0.000_1, 7.4, 0.0],
            [2.0, 3.5, 5.0, 6.5],
        ]);
        let m2 = Matrix([
            [2.1, 3.1, 4.6, 0.9],
            [-1.0, 0.0, -2.4, 7.1],
            [1_261.96, 0.000_1, 7.4, 0.0],
            [2.0, 3.5, 5.0, 6.5],
        ]);
        let m3 = Matrix([
            [2.1, 3.1, 4.6, 0.9],
            [-1.0, 0.0, -2.4, 7.1],
            [1_261.960_01, 0.000_1, 7.4, 0.0],
            [2.0, 3.5, 5.0, 6.5],
        ]);

        assert_approx_eq!(m1, m2);

        assert_approx_ne!(m1, m3);
    }
}
