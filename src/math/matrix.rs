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
}
