use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::EPS;

#[derive(Clone, Copy, Debug)]
pub struct Vector<const T: usize> {
    pub v: [f32; T],
}

pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

impl<const T: usize> Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = -self.v[i];
        }
        res
    }
}

impl<const T: usize> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] + other.v[i];
        }
        res
    }
}

impl<const T: usize> AddAssign for Vector<T> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..T {
            self.v[i] += other.v[i];
        }
    }
}

impl<const T: usize> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] - other.v[i];
        }
        res
    }
}

impl<const T: usize> SubAssign for Vector<T> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..T {
            self.v[i] -= other.v[i];
        }
    }
}

impl<const T: usize> Mul for Vector<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] * other.v[i];
        }
        res
    }
}

impl<const T: usize> Mul<f32> for Vector<T> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] * other;
        }
        res
    }
}

impl<const T: usize> MulAssign<f32> for Vector<T> {
    fn mul_assign(&mut self, other: f32) {
        for i in 0..T {
            self.v[i] *= other;
        }
    }
}

impl<const T: usize> Div for Vector<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] * other.v[i];
        }
        res
    }
}

impl<const T: usize> Div<f32> for Vector<T> {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let mut res = Vector::<T> { v: [0f32; T] };
        for i in 0..T {
            res.v[i] = self.v[i] / other;
        }
        res
    }
}

impl<const T: usize> Vector<T> {
    pub fn dot(&self, other: Self) -> f32 {
        let mut res = 0f32;
        for i in 0..T {
            res += self.v[i] * other.v[i];
        }
        res
    }
    pub fn norm(&self) -> f32 {
        self.v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        *self / norm
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { v: [x, y] }
    }

    pub fn cross(&self, other: Self) -> f32 {
        self.v[0] * other.v[1] - self.v[1] * other.v[0]
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { v: [x, y, z] }
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                self.v[2] * other.v[0] - self.v[0] * other.v[2],
                self.v[0] * other.v[1] - self.v[1] * other.v[0],
            ],
        }
    }

    pub fn homo_point(&self) -> Vector4 {
        Vector4::new(self.v[0], self.v[1], self.v[2], 1.)
    }

    pub fn homo_vec(&self) -> Vector4 {
        Vector4::new(self.v[0], self.v[1], self.v[2], 0.)
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { v: [x, y, z, w] }
    }
    pub fn vec3_homo(&self) -> Vector3 {
        if self.v[3].abs() < EPS {
            Vector3::new(self.v[0], self.v[1], self.v[2])
        } else {
            Vector3::new(
                self.v[0] / self.v[3],
                self.v[1] / self.v[3],
                self.v[2] / self.v[3],
            )
        }
    }
}
