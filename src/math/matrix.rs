use std::{
    fmt::{self, Debug, Formatter},
    ops::{Mul, MulAssign},
    slice::{Iter, IterMut},
};

use anyhow::{bail, Result};
use derive_more::{Index, IndexMut, IntoIterator};
use float_cmp::{ApproxEq, F64Margin};

use super::{float::approx_eq, Point, Vector};

/// A Matrix is a square matrix of size N, stored in row major order.
#[derive(Clone, Copy, Index, IndexMut, IntoIterator)]
pub struct Matrix<const N: usize>(pub(super) [[f64; N]; N]);

impl<const N: usize> Matrix<N> {
    #[must_use]
    fn zero() -> Self {
        Self([[0.0; N]; N])
    }

    #[must_use]
    pub fn identity() -> Self {
        let mut matrix = Self::zero();

        for (index, row_data) in matrix.iter_mut().enumerate() {
            row_data[index] = 1.0;
        }

        matrix
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut matrix = Self::zero();

        for (row, row_data) in self.iter().enumerate() {
            for (col, col_data) in row_data.iter().enumerate() {
                matrix[col][row] = *col_data;
            }
        }

        matrix
    }

    pub fn iter(&self) -> Iter<'_, [f64; N]> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, [f64; N]> {
        self.0.iter_mut()
    }
}

/// Implement submatrix, cofactor, minor and determinant functions for a given
/// matrix size. We would really like to be able to implement these for a given
/// const N size but unfortunately we can't do so for submatrix until const
/// generic exprs is stable (output needs to be Matrix<N - 1>). Since we can't
/// implement submatrix we can't implement minor or cofactor for generic N and
/// we would also need specialisation in stable so we could have determinant for
/// N as well as determinant for Matrix<2>. To avoid lots of code duplication
/// wrap everything in a macro and just implement for Matrix<4> and <3>.
macro_rules! impl_matrix {
    ($size:expr) => {
        impl Matrix<$size> {
            #[must_use]
            pub fn submatrix(
                &self,
                row: usize,
                col: usize,
            ) -> Matrix<{ $size - 1 }> {
                let mut out_matrix = Matrix::zero();

                let mut new_row = 0;
                for (cur_row, row_data) in self.iter().enumerate() {
                    if row == cur_row {
                        continue;
                    }

                    let mut new_col = 0;
                    for (cur_col, value) in row_data.iter().enumerate() {
                        if col == cur_col {
                            continue;
                        }

                        out_matrix[new_row][new_col] = *value;

                        new_col += 1;
                    }

                    new_row += 1;
                }

                out_matrix
            }

            #[must_use]
            pub fn minor(&self, row: usize, col: usize) -> f64 {
                self.submatrix(row, col).determinant()
            }

            #[must_use]
            pub fn cofactor(&self, row: usize, col: usize) -> f64 {
                let minor = self.minor(row, col);

                if (row + col) % 2 != 0 {
                    return minor * -1.0;
                }

                minor
            }

            #[must_use]
            pub fn determinant(&self) -> f64 {
                let mut det = 0.0;

                for col in 0..$size {
                    det += self[0][col] * self.cofactor(0, col);
                }

                det
            }
        }
    };
}

impl_matrix!(4);
impl_matrix!(3);

impl Matrix<4> {
    #[must_use]
    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Self([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn rotate_x(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn rotate_y(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn rotate_z(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();

        Self([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Attempt to invert the matrix.
    ///
    /// # Errors
    ///
    /// Will return an error if the matrix cannot be inverted.
    pub fn invert(&self) -> Result<Self> {
        let det = self.determinant();

        if approx_eq!(det, 0.0) {
            bail!(
                "Tried to invert a Matrix that cannot be inverted - {self:?}"
            );
        }

        let mut matrix = Self::zero();

        for row in 0..4 {
            for col in 0..4 {
                matrix[col][row] = self.cofactor(row, col) / det;
            }
        }

        Ok(matrix)
    }
}

impl Matrix<2> {
    #[must_use]
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Self::Output::zero();

        for (row, row_data) in matrix.iter_mut().enumerate() {
            for (col, value) in row_data.iter_mut().enumerate() {
                for index in 0..N {
                    *value += self[row][index] * rhs[index][col];
                }
            }
        }

        matrix
    }
}

impl<const N: usize> MulAssign for Matrix<N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (*self * rhs).0;
    }
}

impl Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output::new(
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

impl Mul<Matrix<4>> for Point {
    type Output = Self;

    fn mul(self, rhs: Matrix<4>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
        )
    }
}

impl Mul<Matrix<4>> for Vector {
    type Output = Self;

    fn mul(self, rhs: Matrix<4>) -> Self::Output {
        rhs * self
    }
}

impl<'a, const N: usize> IntoIterator for &'a Matrix<N> {
    type Item = &'a [f64; N];

    type IntoIter = Iter<'a, [f64; N]>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut Matrix<N> {
    type Item = &'a mut [f64; N];

    type IntoIter = IterMut<'a, [f64; N]>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize> Debug for Matrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix<{N}>([")?;

        for row_data in self {
            writeln!(f, "    {row_data:?},")?;
        }

        write!(f, "])")
    }
}

impl<const N: usize> ApproxEq for Matrix<N> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        for (lhs_row, rhs_row) in self.iter().zip(other.iter()) {
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
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

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

        assert_approx_eq!(
            Matrix::<4>::identity(),
            Matrix([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );

        assert_approx_eq!(
            Matrix::<3>::zero(),
            Matrix([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        );
    }

    #[test]
    fn multiplying_by_the_identity_matrix() {
        let id = Matrix::identity();

        let m = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_approx_eq!(m * id, m);
        assert_approx_eq!(id * m, m);

        let p = Point::new(1.0, 2.0, 3.0);
        assert_approx_eq!(id * p, p);
        assert_approx_eq!(p * id, p);

        let v = Vector::new(2.0, 3.5, 4.2);
        assert_approx_eq!(id * v, v);
        assert_approx_eq!(v * id, v);
    }

    #[test]
    fn transposing_a_matrix() {
        assert_approx_eq!(
            Matrix([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]
            ])
            .transpose(),
            Matrix([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ])
        );

        let id = Matrix::<3>::identity();
        assert_approx_eq!(id.transpose(), id);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let m = Matrix::translate(5.0, -3.0, 2.0);

        let p = Point::new(-3.0, 4.0, 5.0);
        assert_approx_eq!(m * p, Point::new(2.0, 1.0, 7.0));

        assert_approx_eq!(m.invert().unwrap() * p, Point::new(-8.0, 7.0, 3.0));

        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_approx_eq!(m * v, v);
    }

    #[test]
    fn multiplying_by_a_scaling_matrix() {
        let m = Matrix::scale(2.0, 3.0, 4.0);

        assert_approx_eq!(
            m * Point::new(-4.0, 6.0, 8.0),
            Point::new(-8.0, 18.0, 32.0)
        );

        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_approx_eq!(m * v, Vector::new(-8.0, 18.0, 32.0));

        assert_approx_eq!(m.invert().unwrap() * v, Vector::new(-2.0, 2.0, 2.0));

        assert_approx_eq!(
            Matrix::scale(-1.0, 1.0, 1.0) * Point::new(2.0, 3.0, 4.0),
            Point::new(-2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn multiplying_by_a_rotate_x_matrix() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half = Matrix::rotate_x(FRAC_PI_4);

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            half * p,
            Point::new(0.0, sqrt_2_div_2, sqrt_2_div_2)
        );

        assert_approx_eq!(
            Matrix::rotate_x(FRAC_PI_2) * Vector::y_axis(),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_approx_eq!(
            half.invert().unwrap() * p,
            Point::new(0.0, sqrt_2_div_2, -sqrt_2_div_2)
        );
    }

    #[test]
    fn multiplying_by_a_rotate_y_matrix() {
        let p = Point::new(0.0, 0.0, 1.0);

        let half = Matrix::rotate_y(FRAC_PI_4);

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            half * p,
            Point::new(sqrt_2_div_2, 0.0, sqrt_2_div_2)
        );

        assert_approx_eq!(
            Matrix::rotate_y(FRAC_PI_2) * Vector::z_axis(),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_approx_eq!(
            half.invert().unwrap() * p,
            Point::new(-sqrt_2_div_2, 0.0, sqrt_2_div_2)
        );
    }

    #[test]
    fn multiplying_by_a_rotate_z_matrix() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half = Matrix::rotate_z(FRAC_PI_4);

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            half * p,
            Point::new(-sqrt_2_div_2, sqrt_2_div_2, 0.0)
        );

        assert_approx_eq!(
            Matrix::rotate_z(FRAC_PI_2) * Vector::y_axis(),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_approx_eq!(
            half.invert().unwrap() * p,
            Point::new(sqrt_2_div_2, sqrt_2_div_2, 0.0)
        );
    }

    #[test]
    fn multiplying_by_a_shearing_matrix() {
        let p = Point::new(2.0, 3.0, 4.0);

        assert_approx_eq!(
            Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * p,
            Point::new(5.0, 3.0, 4.0)
        );

        assert_approx_eq!(
            Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * p,
            Point::new(6.0, 3.0, 4.0)
        );

        assert_approx_eq!(
            Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * p,
            Point::new(2.0, 5.0, 4.0)
        );

        assert_approx_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * p,
            Point::new(2.0, 7.0, 4.0)
        );

        assert_approx_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * p,
            Point::new(2.0, 3.0, 6.0)
        );

        assert_approx_eq!(
            Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
                * Vector::new(2.0, 3.0, 4.0),
            Vector::new(2.0, 3.0, 7.0)
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn chaining_multiple_transformations() {
        let o = Point::new(1.0, 0.0, 1.0);

        let r = Matrix::rotate_x(FRAC_PI_2);
        let s = Matrix::scale(5.0, 5.0, 5.0);
        let t = Matrix::translate(10.0, 5.0, 7.0);

        let p = r * o;
        assert_approx_eq!(p, Point::new(1.0, -1.0, 0.0));

        let p = s * p;
        assert_approx_eq!(
            p,
            Point::new(5.0, -5.0, 0.0),
            epsilon = 0.000_000_001
        );

        let p = t * p;
        assert_approx_eq!(p, Point::new(15.0, 0.0, 7.0));

        let m = t * s * r;
        assert_approx_eq!(m * o, p);

        assert_approx_eq!(m.invert().unwrap() * p, o);
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let m = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_approx_eq!(m.determinant(), -2120.0);
        assert!(m.invert().is_ok());

        let m = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let i = m.invert().unwrap();

        assert_approx_eq!(m.determinant(), 532.0);
        assert_approx_eq!(m.cofactor(2, 3), -160.0);
        assert_approx_eq!(i[3][2], -160.0 / 532.0);
        assert_approx_eq!(m.cofactor(3, 2), 105.0);
        assert_approx_eq!(i[2][3], 105.0 / 532.0);

        assert_approx_eq!(
            i,
            Matrix([
                [0.218_05, 0.451_13, 0.240_6, -0.045_11],
                [-0.808_27, -1.456_77, -0.443_61, 0.520_68],
                [-0.078_95, -0.223_68, -0.052_63, 0.197_37],
                [-0.522_56, -0.813_91, -0.300_75, 0.306_39],
            ]),
            epsilon = 0.000_01
        );

        assert_approx_eq!(
            Matrix([
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0],
            ])
            .invert()
            .unwrap(),
            Matrix([
                [-0.153_85, -0.153_85, -0.282_05, -0.538_46],
                [-0.076_92, 0.123_08, 0.025_64, 0.030_77],
                [0.358_97, 0.358_97, 0.435_9, 0.923_08],
                [-0.692_31, -0.692_31, -0.769_23, -1.92308],
            ]),
            epsilon = 0.000_01
        );

        assert_approx_eq!(
            Matrix([
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ])
            .invert()
            .unwrap(),
            Matrix([
                [-0.040_74, -0.077_78, 0.144_44, -0.222_22],
                [-0.077_78, 0.033_33, 0.366_67, -0.333_33],
                [-0.029_01, -0.146_3, -0.109_26, 0.129_63],
                [0.177_78, 0.066_67, -0.266_67, 0.333_33],
            ]),
            epsilon = 0.000_01
        );

        let m1 = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m2 = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let r = m1 * m2;

        assert_approx_eq!(r * m2.invert().unwrap(), m1, epsilon = 0.000_01);
    }

    #[test]
    fn attempt_to_invert_a_matrix_that_cannot_be_inverted() {
        let m = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_approx_eq!(m.determinant(), 0.0);

        let m = m.invert();

        assert!(m.is_err());

        assert_eq!(
            m.err().unwrap().to_string(),
            "\
Tried to invert a Matrix that cannot be inverted - Matrix<4>([
    [-4.0, 2.0, -2.0, -3.0],
    [9.0, 6.0, 2.0, 6.0],
    [0.0, -5.0, 1.0, -5.0],
    [0.0, 0.0, 0.0, 0.0],
])"
        );
    }

    #[test]
    fn calculating_the_submatrix_of_a_matrix() {
        assert_approx_eq!(
            Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]])
                .submatrix(0, 2),
            Matrix([[-3.0, 2.0], [0.0, 6.0]])
        );

        assert_approx_eq!(
            Matrix([
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0]
            ])
            .submatrix(2, 1),
            Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn calculating_the_minor_of_a_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let s = m.submatrix(1, 0);
        assert_approx_eq!(s.determinant(), 25.0);

        assert_approx_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_the_cofactor_of_a_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_approx_eq!(m.minor(0, 0), -12.0);
        assert_approx_eq!(m.cofactor(0, 0), -12.0);
        assert_approx_eq!(m.minor(1, 0), 25.0);
        assert_approx_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_matrix() {
        assert_approx_eq!(
            Matrix([[1.0, 5.0], [-3.0, 2.0]]).determinant(),
            17.0
        );

        let m = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_approx_eq!(m.cofactor(0, 0), 56.0);
        assert_approx_eq!(m.cofactor(0, 1), 12.0);
        assert_approx_eq!(m.cofactor(0, 2), -46.0);
        assert_approx_eq!(m.determinant(), -196.0);

        let m = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx_eq!(m.cofactor(0, 0), 690.0);
        assert_approx_eq!(m.cofactor(0, 1), 447.0);
        assert_approx_eq!(m.cofactor(0, 2), 210.0);
        assert_approx_eq!(m.cofactor(0, 3), 51.0);
        assert_approx_eq!(m.determinant(), -4071.0);
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
    #[should_panic(
        expected = "index out of bounds: the len is 4 but the index is 4"
    )]
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
    fn mutable_indexing_into_a_matrix() {
        let mut m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        m[0][1] = 7.8;
        m[2][0] = 0.7;
        m[3][3] = 12.7;

        assert_approx_eq!(m[0][0], 1.0);
        assert_approx_eq!(m[0][1], 7.8);
        assert_approx_eq!(m[1][2], 7.0);
        assert_approx_eq!(m[2][0], 0.7);
        assert_approx_eq!(m[3][1], 14.0);
        assert_approx_eq!(m[3][3], 12.7);
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 4 but the index is 5"
    )]
    fn mutable_indexing_with_invalid_values() {
        let mut m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [4.0, 3.0, 2.0, 1.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 1.0],
        ]);

        m[5][10] = 0.5;
    }

    #[test]
    fn multiplying_two_matrices() {
        assert_approx_eq!(
            Matrix([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]) * Matrix([
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0]
            ]),
            Matrix([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ])
        );

        let mut m = Matrix([
            [0.2, 0.3, 1.6, -9.2],
            [0.0, 1.0, 1.0, 2.0],
            [5.3, 2.16, -2.5, 6.6],
            [1.0, 1.0, 1.5, 1.5],
        ]);
        m *= Matrix([
            [2.3, 4.5, 6.1, 8.9],
            [1.0, 2.0, 3.0, 4.0],
            [-1.0, -2.0, -3.0, -4.0],
            [-0.0, 0.0, 0.0, 0.0],
        ]);

        assert_approx_eq!(
            m,
            Matrix([
                [-0.84, -1.7, -2.68, -3.42],
                [0.0, 0.0, 0.0, 0.0],
                [16.85, 33.17, 46.31, 65.81],
                [1.8, 3.5, 4.6, 6.9]
            ])
        );
    }

    #[test]
    fn multiplying_a_matrix_by_a_point() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let p = Point::new(1.0, 2.0, 3.0);
        let r = Point::new(18.0, 24.0, 33.0);

        assert_approx_eq!(m * p, r);
        assert_approx_eq!(p * m, r);
    }

    #[test]
    fn multiplying_a_matrix_by_a_vector() {
        let m = Matrix([
            [1.0, 2.0, -2.0, 3.0],
            [0.0, 2.5, 0.1, 0.8],
            [2.4, 4.8, 0.112, -2.5],
            [1.7, 0.6, 2.3, 1.5],
        ]);
        let v = Vector::new(1.5, 2.5, 4.0);
        let r = Vector::new(-1.5, 6.65, 16.048);

        assert_approx_eq!(m * v, r);
        assert_approx_eq!(v * m, r);
    }

    #[test]
    fn iterating_over_a_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let mut c = 1.0;
        for r in m {
            for v in r {
                assert_approx_eq!(v, c);
                c += 1.0;
            }
        }

        let mut c = 1.0;
        for r in &m {
            for v in r {
                assert_approx_eq!(*v, c);
                c += 1.0;
            }
        }

        let mut c = 1.0;
        let mut m = m;
        for r in &mut m {
            for v in r {
                assert_approx_eq!(*v, c);
                c += 1.0;
            }
        }
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
