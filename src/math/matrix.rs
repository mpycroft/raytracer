use std::{
    fmt::Debug,
    ops::{AddAssign, Index, IndexMut, Mul, MulAssign},
};

use anyhow::{bail, Result};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use derive_more::Constructor;
use num_traits::{Float, FromPrimitive};

use super::{
    approx::{FLOAT_EPSILON, FLOAT_ULPS},
    Angle, Point, Vector,
};

/// A Matrix is a square matrix of size T, stored in row major order. Due to the
/// limitations on current const generics the implementation is a bit haphazard.
/// The basics like creation, transpose and multiplication should work on
/// arbitrary matrices but determinants, sub matrices, cofactors, etc. are only
/// implemented enough for what we need to work.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Matrix<T: Float, const S: usize> {
    data: [[T; S]; S],
}

impl<T: Float, const S: usize> Matrix<T, S> {
    fn zero() -> Self {
        Self::new([[T::zero(); S]; S])
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = Self::zero();

        for row in 0..S {
            for col in 0..S {
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
fn calc_sub_matrix<T: Float, const S: usize, const R: usize>(
    matrix: &Matrix<T, S>,
    row: usize,
    col: usize,
) -> Matrix<T, R> {
    let mut sub_matrix = Matrix::zero();

    let mut new_row = 0;
    for cur_row in 0..S {
        if cur_row == row {
            continue;
        }

        let mut new_col = 0;
        for cur_col in 0..S {
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
            return minor * -T::one();
        }

        minor
    }};
}

macro_rules! calc_determinant {
    ($self:ident, $size:expr) => {{
        let mut det = T::zero();

        for col in 0..$size {
            det += $self[0][col] * $self.cofactor(0, col);
        }

        det
    }};
}

impl<T> Matrix<T, 4>
where
    T: Float + AddAssign + Debug + RelativeEq,
    T::Epsilon: FromPrimitive + Copy,
{
    pub fn identity() -> Self {
        Self::new([
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn view_transform(
        from: &Point<T>,
        to: &Point<T>,
        up: &Vector<T>,
    ) -> Self {
        let forward = (*to - *from).normalise();
        let up = up.normalise();
        let left = forward.cross(&up);
        let true_up = left.cross(&forward);

        let orientation = Matrix::new([
            [left.x, left.y, left.z, T::zero()],
            [true_up.x, true_up.y, true_up.z, T::zero()],
            [-forward.x, -forward.y, -forward.z, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ]);

        orientation * Matrix::translate(-from.x, -from.y, -from.z)
    }

    pub fn invert(&self) -> Result<Self> {
        let det = self.determinant();

        if float_relative_eq!(det, T::zero()) {
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

    pub fn rotate_x(angle: Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new([
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), cos, -sin, T::zero()],
            [T::zero(), sin, cos, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn rotate_y(angle: Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new([
            [cos, T::zero(), sin, T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [-sin, T::zero(), cos, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn rotate_z(angle: Angle<T>) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new([
            [cos, -sin, T::zero(), T::zero()],
            [sin, cos, T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn scale(x: T, y: T, z: T) -> Self {
        Self::new([
            [x, T::zero(), T::zero(), T::zero()],
            [T::zero(), y, T::zero(), T::zero()],
            [T::zero(), T::zero(), z, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
        Self::new([
            [T::one(), xy, xz, T::zero()],
            [yx, T::one(), yz, T::zero()],
            [zx, zy, T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn translate(x: T, y: T, z: T) -> Self {
        Self::new([
            [T::one(), T::zero(), T::zero(), x],
            [T::zero(), T::one(), T::zero(), y],
            [T::zero(), T::zero(), T::one(), z],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    pub fn cofactor(&self, row: usize, col: usize) -> T {
        calc_cofactor!(self, row, col)
    }

    pub fn determinant(&self) -> T {
        calc_determinant!(self, 4)
    }

    pub fn minor(&self, row: usize, col: usize) -> T {
        calc_minor!(self, row, col)
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<T, 3> {
        calc_sub_matrix(self, row, col)
    }
}

impl<T: Float + AddAssign> Matrix<T, 3> {
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        calc_cofactor!(self, row, col)
    }

    pub fn determinant(&self) -> T {
        calc_determinant!(self, 3)
    }

    pub fn minor(&self, row: usize, col: usize) -> T {
        calc_minor!(self, row, col)
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<T, 2> {
        calc_sub_matrix(self, row, col)
    }
}

impl<T: Float> Matrix<T, 2> {
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<T: Float, const S: usize> Index<usize> for Matrix<T, S> {
    type Output = [T; S];

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T: Float, const S: usize> IndexMut<usize> for Matrix<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T: Float, const S: usize> Mul for Matrix<T, S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Self::Output::new([[T::zero(); S]; S]);

        for row in 0..S {
            for col in 0..S {
                matrix[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }

        matrix
    }
}

impl<T: Float> Mul<Point<T>> for Matrix<T, 4> {
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
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

impl<T: Float> Mul<Vector<T>> for Matrix<T, 4> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
        )
    }
}

impl<T: Float, const S: usize> MulAssign for Matrix<T, S> {
    fn mul_assign(&mut self, rhs: Self) {
        let lhs = *self;

        for row in 0..S {
            for col in 0..S {
                self[row][col] = lhs[row][0] * rhs[0][col]
                    + lhs[row][1] * rhs[1][col]
                    + lhs[row][2] * rhs[2][col]
                    + lhs[row][3] * rhs[3][col];
            }
        }
    }
}

impl<T, const S: usize> AbsDiffEq for Matrix<T, S>
where
    T: Float + AbsDiffEq,
    T::Epsilon: FromPrimitive + Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for row in 0..S {
            for col in 0..S {
                if !self[row][col].abs_diff_eq(&other[row][col], epsilon) {
                    return false;
                }
            }
        }

        true
    }
}

impl<T, const S: usize> RelativeEq for Matrix<T, S>
where
    T: Float + RelativeEq,
    T::Epsilon: FromPrimitive + Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        for row in 0..S {
            for col in 0..S {
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

impl<T, const S: usize> UlpsEq for Matrix<T, S>
where
    T: Float + UlpsEq,
    T::Epsilon: FromPrimitive + Copy,
{
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        for row in 0..S {
            for col in 0..S {
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
    fn constructing_and_inspecting_a_4x4_matrix() {
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
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let m =
            Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_float_relative_eq!(m[0][0], -3.0);
        assert_float_relative_eq!(m[1][1], -2.0);
        assert_float_relative_eq!(m[2][2], 1.0);
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_float_relative_eq!(m[0][0], -3.0);
        assert_float_relative_eq!(m[0][1], 5.0);
        assert_float_relative_eq!(m[1][0], 1.0);
        assert_float_relative_eq!(m[1][1], -2.0);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let m = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_relative_eq!(m * Matrix::identity(), m);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_point() {
        let p = Point::new(1.3, 4.5, 0.9);

        assert_relative_eq!(Matrix::identity() * p, p);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_vector() {
        let v = Vector::new(-3.5, 0.0, 1.8);

        assert_relative_eq!(Matrix::identity() * v, v);
    }

    #[test]
    fn transposing_a_matrix() {
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
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let m = Matrix::<f64, 4>::identity();

        assert_relative_eq!(m.transpose(), m);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_float_relative_eq!(m.determinant(), -2120.0);
        assert!(m.invert().is_ok());
    }

    #[test]
    fn testing_a_non_invertible_matrix_for_invertibility() {
        let m = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_float_relative_eq!(m.determinant(), 0.0);
        assert!(m.invert().is_err());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
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
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
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
    fn multiplying_by_a_translation_matrix() {
        assert_relative_eq!(
            Matrix::translate(5.0, -3.0, 2.0) * Point::new(-3.0, 4.0, 5.0),
            Point::new(2.0, 1.0, 7.0)
        );
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        assert_relative_eq!(
            Matrix::translate(5.0, -3.0, 2.0).invert().unwrap()
                * Point::new(-3.0, 4.0, 5.0),
            Point::new(-8.0, 7.0, 3.0)
        );
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_relative_eq!(Matrix::translate(5.0, -3.0, 2.0) * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        assert_relative_eq!(
            Matrix::scale(2.0, 3.0, 4.0) * Point::new(-4.0, 6.0, 8.0),
            Point::new(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        assert_relative_eq!(
            Matrix::scale(2.0, 3.0, 4.0) * Vector::new(-4.0, 6.0, 8.0),
            Vector::new(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        assert_relative_eq!(
            Matrix::scale(2.0, 3.0, 4.0).invert().unwrap()
                * Vector::new(-4.0, 6.0, 8.0),
            Vector::new(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        assert_relative_eq!(
            Matrix::scale(-1.0, 1.0, 1.0) * Point::new(2.0, 3.0, 4.0),
            Point::new(-2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        assert_relative_eq!(
            Matrix::rotate_x(Angle::from_radians(FRAC_PI_4))
                * Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
        );

        assert_relative_eq!(
            Matrix::rotate_x(Angle::from_radians(FRAC_PI_2)) * Vector::y_axis(),
            Vector::z_axis()
        );
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        assert_relative_eq!(
            Matrix::rotate_x(Angle::from_radians(FRAC_PI_4)).invert().unwrap()
                * Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        assert_relative_eq!(
            Matrix::rotate_y(Angle::from_degrees(45.0))
                * Point::new(0.0, 0.0, 1.0),
            Point::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );

        assert_relative_eq!(
            Matrix::rotate_y(Angle::from_radians(FRAC_PI_2)) * Vector::z_axis(),
            Vector::x_axis()
        );
    }

    #[test]
    fn the_inverse_of_an_y_rotation_rotates_in_the_opposite_direction() {
        assert_relative_eq!(
            Matrix::rotate_y(Angle::from_degrees(45.0)).invert().unwrap()
                * Point::new(0.0, 0.0, 1.0),
            Point::new(-FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        assert_relative_eq!(
            Matrix::rotate_z(Angle::from_radians(FRAC_PI_4))
                * Point::new(0.0, 1.0, 0.0),
            Point::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );

        assert_relative_eq!(
            Matrix::rotate_z(Angle::from_degrees(90.0)) * Vector::y_axis(),
            -Vector::x_axis()
        );
    }

    #[test]
    fn the_inverse_of_an_z_rotation_rotates_in_the_opposite_direction() {
        assert_relative_eq!(
            Matrix::rotate_z(Angle::from_radians(FRAC_PI_4)).invert().unwrap()
                * Point::new(0.0, 1.0, 0.0),
            Point::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        assert_relative_eq!(
            Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(5.0, 3.0, 4.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        assert_relative_eq!(
            Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(6.0, 3.0, 4.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(2.0, 5.0, 4.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(2.0, 7.0, 4.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(2.0, 3.0, 6.0)
        );
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        assert_relative_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
                * Point::new(2.0, 3.0, 4.0),
            Point::new(2.0, 3.0, 7.0)
        );
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let mut p = Matrix::rotate_x(Angle::from_radians(FRAC_PI_2))
            * Point::new(1.0, 0.0, 1.0);
        assert_relative_eq!(p, Point::new(1.0, -1.0, 0.0));

        p = Matrix::scale(5.0, 5.0, 5.0) * p;
        assert_relative_eq!(p, Point::new(5.0, -5.0, 0.0));

        p = Matrix::translate(10.0, 5.0, 7.0) * p;
        assert_relative_eq!(p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let chain = Matrix::translate(10.0, 5.0, 7.0)
            * Matrix::scale(5.0, 5.0, 5.0)
            * Matrix::rotate_x(Angle::from_radians(FRAC_PI_2));

        assert_relative_eq!(
            chain * Point::new(1.0, 0.0, 1.0),
            Point::new(15.0, 0.0, 7.0)
        );
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        assert_relative_eq!(
            Matrix::view_transform(
                &Point::origin(),
                &Point::new(0.0, 0.0, -1.0),
                &Vector::y_axis()
            ),
            Matrix::identity()
        );
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_the_positive_z_direction() {
        assert_relative_eq!(
            Matrix::view_transform(
                &Point::origin(),
                &Point::new(0.0, 0.0, 1.0),
                &Vector::y_axis()
            ),
            Matrix::scale(-1.0, 1.0, -1.0)
        );
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        assert_relative_eq!(
            Matrix::view_transform(
                &Point::new(0.0, 0.0, 8.0),
                &Point::origin(),
                &Vector::y_axis()
            ),
            Matrix::translate(0.0, 0.0, -8.0)
        );
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        assert_relative_eq!(
            Matrix::view_transform(
                &Point::new(1.0, 3.0, 2.0),
                &Point::new(4.0, -2.0, 8.0),
                &Vector::new(1.0, 1.0, 0.0)
            ),
            Matrix::new([
                [-0.507_093, 0.507_093, 0.676_123, -2.366_432],
                [0.767_716, 0.606_092, 0.121_218, -2.828_427],
                [-0.358_569, 0.597_614, -0.717_137, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        assert_float_relative_eq!(
            Matrix::new([[1.0, 5.0], [-3.0, 2.0]]).determinant(),
            17.0
        );
    }

    #[test]
    fn a_sub_matrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        assert_relative_eq!(
            Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]])
                .sub_matrix(0, 2),
            Matrix::new([[-3.0, 2.0], [0.0, 6.0]])
        );
    }

    #[test]
    fn a_sub_matrix_of_a_4x4_matrix_is_a_3x3_matrix() {
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
    fn calculating_a_minor_of_a_3x3_matrix() {
        let m =
            Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_float_relative_eq!(m.sub_matrix(1, 0).determinant(), 25.0);
        assert_float_relative_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let m =
            Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_float_relative_eq!(m.minor(0, 0), -12.0);
        assert_float_relative_eq!(m.cofactor(0, 0), -12.0);

        assert_float_relative_eq!(m.minor(1, 0), 25.0);
        assert_float_relative_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let m =
            Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_float_relative_eq!(m.cofactor(0, 0), 56.0);
        assert_float_relative_eq!(m.cofactor(0, 1), 12.0);
        assert_float_relative_eq!(m.cofactor(0, 2), -46.0);

        assert_float_relative_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
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
    fn multiplying_two_matrices() {
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
    fn matrix_multiplied_by_a_point() {
        assert_relative_eq!(
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]) * Point::new(1.0, 2.0, 3.0),
            Point::new(18.0, 24.0, 33.0)
        );
    }

    #[test]
    fn matrix_multiplied_by_a_vector() {
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
    fn matrices_are_approximately_equal() {
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
