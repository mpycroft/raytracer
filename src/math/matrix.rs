use std::ops::{Index, IndexMut, Mul, MulAssign};

use anyhow::{bail, Result};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use derive_more::Constructor;

use super::{
    approx::{FLOAT_EPSILON, FLOAT_ULPS},
    Point, Vector,
};

/// A Matrix is a square matrix of size T, stored in row major order. Due to the
/// limitations on current const generics the implementation is a bit haphazard.
/// The basics like creation, transpose and multiplication should work on
/// arbitrary matrices but determinants, sub matrices, cofactors, etc. are only
/// implemented enough for what we need to work.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Matrix<const T: usize> {
    data: [[f64; T]; T],
}

impl<const T: usize> Matrix<T> {
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

macro_rules! calc_cofactor {
    ($self:ident, $row:ident, $col:ident) => {{
        let minor = $self.minor($row, $col);

        if ($row + $col) % 2 != 0 {
            return minor * -1.0;
        }

        minor
    }};
}

macro_rules! determinant {
    ($self:ident, $size:expr) => {{
        let mut det = 0.0;

        for col in 0..$size {
            det += $self[0][col] * $self.cofactor(0, col);
        }

        det
    }};
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

    pub fn rotate_x(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_y(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self::new([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_z(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self::new([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Self::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        calc_cofactor!(self, row, col)
    }

    pub fn determinant(&self) -> f64 {
        determinant!(self, 4)
    }

    pub fn invert(&self) -> Result<Self> {
        let det = self.determinant();

        if float_relative_eq!(det, 0.0) {
            bail!("Tried to invert a non invertible matrix - {:?}", self);
        }

        let mut matrix = Matrix::zero();

        for row in 0..4 {
            for col in 0..4 {
                matrix[col][row] = self.cofactor(row, col) / det;
            }
        }

        Ok(matrix)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        calc_minor!(self, row, col)
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<3> {
        sub_matrix(self, row, col)
    }
}

impl Matrix<3> {
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        calc_cofactor!(self, row, col)
    }

    pub fn determinant(&self) -> f64 {
        determinant!(self, 3)
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
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    use approx::*;

    use super::*;

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
    fn rotate_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let m = Matrix::rotate_x(FRAC_PI_4);

        assert_relative_eq!(
            m * p,
            Point::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
        );

        assert_relative_eq!(
            Matrix::rotate_x(FRAC_PI_2) * Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_relative_eq!(
            m.invert().unwrap() * p,
            Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn rotate_y() {
        let p = Point::new(0.0, 0.0, 1.0);
        let m = Matrix::rotate_y(FRAC_PI_4);

        assert_relative_eq!(
            m * p,
            Point::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );

        assert_relative_eq!(
            Matrix::rotate_y(FRAC_PI_2) * Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_relative_eq!(
            m.invert().unwrap() * p,
            Point::new(-FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn rotate_z() {
        let p = Point::new(0.0, 1.0, 0.0);
        let m = Matrix::rotate_z(FRAC_PI_4);

        assert_relative_eq!(
            m * p,
            Point::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );

        assert_relative_eq!(
            Matrix::rotate_z(FRAC_PI_2) * Vector::new(0.0, 1.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_relative_eq!(
            m.invert().unwrap() * p,
            Point::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );
    }

    #[test]
    fn scale() {
        let m = Matrix::scale(2.0, 3.0, 4.0);

        assert_relative_eq!(
            m * Point::new(-4.0, 6.0, 8.0),
            Point::new(-8.0, 18.0, 32.0)
        );

        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_relative_eq!(m * v, Vector::new(-8.0, 18.0, 32.0));

        assert_relative_eq!(
            m.invert().unwrap() * v,
            Vector::new(-2.0, 2.0, 2.0)
        );

        assert_relative_eq!(
            Matrix::scale(-1.0, 1.0, 1.0) * Point::new(2.0, 3.0, 4.0),
            Point::new(-2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn shear() {
        let p = Point::new(2.0, 3.0, 4.0);

        assert_relative_eq!(
            Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * p,
            Point::new(5.0, 3.0, 4.0)
        );
        assert_relative_eq!(
            Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * p,
            Point::new(6.0, 3.0, 4.0)
        );

        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * p,
            Point::new(2.0, 5.0, 4.0)
        );
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * p,
            Point::new(2.0, 7.0, 4.0)
        );

        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * p,
            Point::new(2.0, 3.0, 6.0)
        );
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * p,
            Point::new(2.0, 3.0, 7.0)
        );

        let m = Matrix::shear(1.0, 2.9, 0.0, 1.0, 2.5, 5.2);
        assert_relative_eq!(m.invert().unwrap() * m * p, p);
    }

    #[test]
    fn translate() {
        let m = Matrix::translate(5.0, -3.0, 2.0);

        assert_relative_eq!(
            m * Point::new(-3.0, 4.0, 5.0),
            Point::new(2.0, 1.0, 7.0)
        );

        assert_relative_eq!(
            m.invert().unwrap() * Point::new(-3.0, 4.0, 5.0),
            Point::new(-8.0, 7.0, 3.0)
        );

        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_relative_eq!(m * v, v);
    }

    #[test]
    fn chaining_transforms() {
        let point = Point::new(1.0, 0.0, 1.0);
        let final_point = Point::new(15.0, 0.0, 7.0);

        let rotate = Matrix::rotate_x(FRAC_PI_2);
        let scale = Matrix::scale(5.0, 5.0, 5.0);
        let translate = Matrix::translate(10.0, 5.0, 7.0);

        let mut p = rotate * point;
        assert_relative_eq!(p, Point::new(1.0, -1.0, 0.0));

        p = scale * p;
        assert_relative_eq!(p, Point::new(5.0, -5.0, 0.0));

        p = translate * p;
        assert_relative_eq!(p, final_point);

        let chain = translate * scale * rotate;

        assert_relative_eq!(chain * point, final_point);

        assert_relative_eq!(chain.invert().unwrap() * final_point, point);
    }

    #[test]
    fn cofactor() {
        let m =
            Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_float_relative_eq!(m.minor(0, 0), -12.0);
        assert_float_relative_eq!(m.cofactor(0, 0), -12.0);

        assert_float_relative_eq!(m.minor(1, 0), 25.0);
        assert_float_relative_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant() {
        assert_float_relative_eq!(
            Matrix::new([[1.0, 5.0], [-3.0, 2.0]]).determinant(),
            17.0
        );

        let m =
            Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_float_relative_eq!(m.cofactor(0, 0), 56.0);
        assert_float_relative_eq!(m.cofactor(0, 1), 12.0);
        assert_float_relative_eq!(m.cofactor(0, 2), -46.0);

        assert_float_relative_eq!(m.determinant(), -196.0);

        let m = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_float_relative_eq!(m.cofactor(0, 0), 690.0);
        assert_float_relative_eq!(m.cofactor(0, 1), 447.0);
        assert_float_relative_eq!(m.cofactor(0, 2), 210.0);
        assert_float_relative_eq!(m.cofactor(0, 3), 51.0);

        assert_float_relative_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn invert() {
        let m = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_float_relative_eq!(m.determinant(), -2120.0);
        assert!(m.invert().is_ok());

        let m = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_float_relative_eq!(m.determinant(), 0.0);
        assert!(m.invert().is_err());

        let m = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let inverse = m.invert().unwrap();

        assert_float_relative_eq!(m.determinant(), 532.0);

        assert_float_relative_eq!(m.cofactor(2, 3), -160.0);
        assert_float_relative_eq!(inverse[3][2], -0.300_752);

        assert_float_relative_eq!(m.cofactor(3, 2), 105.0);
        assert_float_relative_eq!(inverse[2][3], 0.197_368);

        assert_relative_eq!(
            inverse,
            Matrix::new([
                [0.218_045, 0.451_128, 0.240_602, -0.045_113],
                [-0.808_271, -1.456_767, -0.443_609, 0.520_677],
                [-0.078_947, -0.223_684, -0.052_632, 0.197_368],
                [-0.522_556, -0.813_91, -0.300_752, 0.306_391]
            ])
        );

        assert_relative_eq!(
            Matrix::new([
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0]
            ])
            .invert()
            .unwrap(),
            Matrix::new([
                [-0.153_846, -0.153_846, -0.282_051, -0.538_461],
                [-0.076_923, 0.123_077, 0.025_641, 0.030_769],
                [0.358_974, 0.358_974, 0.435_897, 0.923_077],
                [-0.692_308, -0.692_308, -0.769_23, -1.923_077]
            ])
        );

        assert_relative_eq!(
            Matrix::new([
                [9.0, 3.0, 0.0, 9.0,],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0,],
                [-7.0, 6.0, 6.0, 2.0]
            ])
            .invert()
            .unwrap(),
            Matrix::new([
                [-0.040_741, -0.077_778, 0.144_444, -0.222_222],
                [-0.077_778, 0.033_333, 0.366_667, -0.333_333],
                [-0.029_012, -0.146_296, -0.109_26, 0.129_63],
                [0.177_778, 0.066_667, -0.266_667, 0.333_333]
            ])
        );

        let m1 = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let m2 = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let m3 = m1 * m2;

        assert_relative_eq!(m3 * m2.invert().unwrap(), m1);
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
