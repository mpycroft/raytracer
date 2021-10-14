use super::float::{FLOAT_EPSILON, FLOAT_ULPS};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::Index;

/// A Matrix is a square matrix of size T, stored in row major order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const T: usize> {
    data: [[f64; T]; T],
}

impl<const T: usize> Matrix<T> {
    pub fn new(data: [[f64; T]; T]) -> Self {
        Self { data }
    }
}

impl<const T: usize> Index<usize> for Matrix<T> {
    type Output = [f64; T];

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const T: usize> AbsDiffEq for Matrix<T> {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for row in 0..T {
            for col in 0..T {
                if !self[row][col].abs_diff_eq(&other[row][col], epsilon) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const T: usize> RelativeEq for Matrix<T> {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        for row in 0..T {
            for col in 0..T {
                if !self[row][col].relative_eq(
                    &other[row][col],
                    epsilon,
                    max_relative,
                ) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const T: usize> UlpsEq for Matrix<T> {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        for row in 0..T {
            for col in 0..T {
                if !self[row][col].ulps_eq(&other[row][col], epsilon, max_ulps)
                {
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
    use approx::*;

    #[test]
    fn new() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_float_relative_eq!(m[0][0], 1.0);
        assert_float_relative_eq!(m[0][3], 4.0);
        assert_float_relative_eq!(m[1][0], 5.5);
        assert_float_relative_eq!(m[1][2], 7.5);
        assert_float_relative_eq!(m[2][2], 11.0);
        assert_float_relative_eq!(m[3][0], 13.5);
        assert_float_relative_eq!(m[3][2], 15.5);

        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_float_relative_eq!(m[0][0], -3.0);
        assert_float_relative_eq!(m[0][1], 5.0);
        assert_float_relative_eq!(m[1][0], 1.0);
        assert_float_relative_eq!(m[1][1], -2.0);

        let m =
            Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_float_relative_eq!(m[0][0], -3.0);
        assert_float_relative_eq!(m[1][1], -2.0);
        assert_float_relative_eq!(m[2][2], 1.0);
    }

    #[test]
    fn approx() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m3 = Matrix::new([
            [1.000_01, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.000_6, 3.0, 2.0],
        ]);
        let m4 = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_abs_diff_eq!(m1, m2);
        assert_abs_diff_ne!(m1, m3);
        assert_abs_diff_ne!(m1, m4);

        assert_relative_eq!(m1, m2);
        assert_relative_ne!(m1, m3);
        assert_relative_ne!(m1, m4);

        assert_ulps_eq!(m1, m2);
        assert_ulps_ne!(m1, m3);
        assert_ulps_ne!(m1, m4);
    }
}
