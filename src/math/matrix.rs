use super::{
    float::{FLOAT_EPSILON, FLOAT_ULPS},
    Point, Vector,
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Index, IndexMut, Mul, MulAssign};

/// A Matrix is a square matrix of size T, stored in row major order. Due to the
/// limitations on current const generics the implementation is a bit haphazard.
/// The basics like creation, transpose and multiplication should work on
/// arbitrary matrices but determinants, sub matrices, cofactors, etc. are only
/// implemented enough for what we need to work.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const T: usize> {
    data: [[f64; T]; T],
}

impl<const T: usize> Matrix<T> {
    pub fn new(data: [[f64; T]; T]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self::new([[0.0; T]; T])
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = Self::zero();

        for row in 0..T {
            for col in 0..T {
                matrix[row][col] = self[col][row];
            }
        }

        matrix
    }
}

// Unfortunately due to the limited nature of const generics currently there is
// not a very nice way to handle this type of function. Ideally we would just
// have this in the general impl with -> Matrix<T - 1> but this is not valid
// (yet). We can constrain the function with an extra U parameter like we do
// here in the general impl but since this gets called recursively for
// Matrix<4>'s it ends up not reducing down correctly (at least not without
// introducing even more parameters). Therefore this seems like the best
// solution; a general function and only implement it for Matrix<3/4> since that
// is all we actually need.
#[inline(always)]
fn sub_matrix<const T: usize, const U: usize>(
    matrix: &Matrix<T>,
    row: usize,
    col: usize,
) -> Matrix<U> {
    let mut sub_matrix = Matrix::zero();

    let mut new_row = 0;
    for cur_row in 0..T {
        if cur_row == row {
            continue;
        }

        let mut new_col = 0;
        for cur_col in 0..T {
            if cur_col == col {
                continue;
            }

            sub_matrix[new_row][new_col] = matrix[cur_row][cur_col];

            new_col += 1;
        }

        new_row += 1;
    }

    sub_matrix
}

// Macros to reduce (very minor) code duplication, we could use functions like
// calc_sub_matrix but we'd need to make sub_matrix (and possibly others) traits
// which is a lot of effort for what we are doing here.
macro_rules! calc_minor {
    ($self:ident, $row:ident, $col:ident) => {
        $self.sub_matrix($row, $col).determinant()
    };
}

impl Matrix<4> {
    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn determinant(&self) -> f64 {
        todo!()
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        calc_minor!(self, row, col)
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<3> {
        sub_matrix(self, row, col)
    }
}

impl Matrix<3> {
    pub fn determinant(&self) -> f64 {
        todo!()
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        calc_minor!(self, row, col)
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<2> {
        sub_matrix(self, row, col)
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<const T: usize> Index<usize> for Matrix<T> {
    type Output = [f64; T];

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const T: usize> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<const T: usize> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Self::Output::new([[0.0; T]; T]);

        for row in 0..T {
            for col in 0..T {
                matrix[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }

        matrix
    }
}

impl Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            self[0][0] * rhs.x
                + self[0][1] * rhs.y
                + self[0][2] * rhs.z
                + self[0][3],
            self[1][0] * rhs.x
                + self[1][1] * rhs.y
                + self[1][2] * rhs.z
                + self[1][3],
            self[2][0] * rhs.x
                + self[2][1] * rhs.y
                + self[2][2] * rhs.z
                + self[2][3],
        )
    }
}

impl Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
        )
    }
}

impl<const T: usize> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let lhs = *self;

        for row in 0..T {
            for col in 0..T {
                self[row][col] = lhs[row][0] * rhs[0][col]
                    + lhs[row][1] * rhs[1][col]
                    + lhs[row][2] * rhs[2][col]
                    + lhs[row][3] * rhs[3][col];
            }
        }
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
    fn determinant() {
        assert_float_relative_eq!(
            Matrix::new([[1.0, 5.0], [-3.0, 2.0]]).determinant(),
            17.0
        );
    }

    #[test]
    fn identity() {
        let identity = Matrix::identity();

        let m = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_relative_eq!(m * identity, m);

        let p = Point::new(1.3, 4.5, 0.9);

        assert_relative_eq!(identity * p, p);

        let v = Vector::new(-3.5, 0.0, 1.8);

        assert_relative_eq!(identity * v, v);
    }

    #[test]
    fn minor() {
        let m =
            Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_float_relative_eq!(m.sub_matrix(1, 0).determinant(), 25.0);
        assert_float_relative_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn sub_matrix() {
        assert_relative_eq!(
            Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]])
                .sub_matrix(0, 2),
            Matrix::new([[-3.0, 2.0], [0.0, 6.0]])
        );

        assert_relative_eq!(
            Matrix::new([
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0]
            ])
            .sub_matrix(2, 1),
            Matrix::new([
                [-6.0, 1.0, 6.0],
                [-8.0, 8.0, 6.0],
                [-7.0, -1.0, 1.0]
            ])
        );
    }

    #[test]
    fn transpose() {
        assert_relative_eq!(
            Matrix::new([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]
            ])
            .transpose(),
            Matrix::new([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ])
        );

        assert_relative_eq!(Matrix::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn mul() {
        assert_relative_eq!(
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]) * Matrix::new([
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ]),
            Matrix::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ])
        );

        assert_relative_eq!(
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]) * Point::new(1.0, 2.0, 3.0),
            Point::new(18.0, 24.0, 33.0)
        );

        assert_relative_eq!(
            Matrix::new([
                [1.0, 2.0, -2.0, 3.0],
                [0.0, 2.5, 0.1, 0.8],
                [2.4, 4.8, 0.112, -2.5],
                [1.7, 0.6, 2.3, 1.5]
            ]) * Vector::new(1.5, 2.5, 4.0),
            Vector::new(-1.5, 6.65, 16.048)
        );
    }

    #[test]
    fn mul_assign() {
        let mut m = Matrix::new([
            [1.3, 0.5, 3.4, 12.0],
            [0.0, 0.9, 0.8, 2.11],
            [6.9, 12.3, 11.0, 10.9],
            [1.0, 2.0, 3.4, 3.1],
        ]);
        m *= Matrix::new([
            [4.1, 4.2, 0.88, -6.1],
            [1.3, 4.2, -2.1, 2.8],
            [2.2, 2.3, 1.6, 25.0],
            [0.0, 0.0, 2.1, -5.1],
        ]);

        assert_relative_eq!(
            m,
            Matrix::new([
                [13.46, 15.38, 30.734, 17.27],
                [2.93, 5.62, 3.821, 11.759],
                [68.48, 105.94, 20.732, 211.76],
                [14.18, 20.42, 8.63, 68.69]
            ])
        );
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
