use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const R: usize, const C: usize> {
    pub v: [[f32; C]; R],
}

impl<const R: usize, const C: usize> Add for Matrix<R, C> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Matrix::<R, C> { v: [[0f32; C]; R] };
        for i in 0..R {
            for j in 0..C {
                res.v[i][j] = self.v[i][j] + other.v[i][j];
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> AddAssign for Matrix<R, C> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.v[i][j] += other.v[i][j];
            }
        }
    }
}

impl<const R: usize, const C: usize> Sub for Matrix<R, C> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = Matrix::<R, C> { v: [[0f32; C]; R] };
        for i in 0..R {
            for j in 0..C {
                res.v[i][j] = self.v[i][j] - other.v[i][j];
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> SubAssign for Matrix<R, C> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.v[i][j] += other.v[i][j];
            }
        }
    }
}

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;

impl<const R: usize, const C: usize, const W: usize> Mul<Matrix<C, W>> for Matrix<R, C> {
    type Output = Matrix<R, W>;

    fn mul(self, other: Matrix<C, W>) -> Matrix<R, W> {
        let mut res = Matrix::<R, W> { v: [[0f32; W]; R] };
        for i in 0..R {
            for j in 0..C {
                for k in 0..W {
                    res.v[i][k] += self.v[i][j] * other.v[j][k];
                }
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> Mul<Vector<C>> for Matrix<R, C> {
    type Output = Vector<R>;

    fn mul(self, other: Vector<C>) -> Vector<R> {
        let mut res = Vector::<R> { v: [0f32; R] };
        for i in 0..R {
            for j in 0..C {
                res.v[i] += self.v[i][j] * other.v[j];
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> Mul<f32> for Matrix<R, C> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut res = Matrix::<R, C> { v: [[0f32; C]; R] };
        for i in 0..R {
            for j in 0..C {
                res.v[i][j] = self.v[i][j] * other;
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> MulAssign<f32> for Matrix<R, C> {
    fn mul_assign(&mut self, other: f32) {
        for i in 0..R {
            for j in 0..C {
                self.v[i][j] *= other;
            }
        }
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zeros() -> Matrix<R, C> {
        Matrix::<R, C> { v: [[0f32; C]; R] }
    }
    pub fn identity() -> Matrix<R, C> {
        let mut res = Matrix::<R, C> { v: [[0f32; C]; R] };
        for i in 0..R {
            res.v[i][i] = 1.0;
        }
        res
    }
}
