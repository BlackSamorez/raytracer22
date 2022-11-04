use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Default, PartialEq, Deserialize, Serialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Clone for Vector3D {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl From<[f64; 3]> for Vector3D {
    fn from(array: [f64; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<R> std::ops::Add<R> for Vector3D
where
    R: Borrow<Vector3D>,
{
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        Self {
            x: self.x + rhs.borrow().x,
            y: self.y + rhs.borrow().y,
            z: self.z + rhs.borrow().z,
        }
    }
}

impl<R> std::ops::Add<R> for &Vector3D
where
    R: Borrow<Vector3D>,
{
    type Output = Vector3D;

    fn add(self, rhs: R) -> Self::Output {
        Self::Output {
            x: self.x + rhs.borrow().x,
            y: self.y + rhs.borrow().y,
            z: self.z + rhs.borrow().z,
        }
    }
}

impl<R> std::ops::AddAssign<R> for Vector3D
where
    R: Borrow<Vector3D>,
{
    fn add_assign(&mut self, rhs: R) {
        self.x += rhs.borrow().x;
        self.y += rhs.borrow().y;
        self.z += rhs.borrow().z;
    }
}

impl std::ops::Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Neg for &Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<R> std::ops::Sub<R> for Vector3D
where
    R: Borrow<Vector3D>,
{
    type Output = Vector3D;

    fn sub(self, rhs: R) -> Self::Output {
        Self::Output {
            x: self.x - rhs.borrow().x,
            y: self.y - rhs.borrow().y,
            z: self.z - rhs.borrow().z,
        }
    }
}

impl<R> std::ops::Sub<R> for &Vector3D
where
    R: Borrow<Vector3D>,
{
    type Output = Vector3D;

    fn sub(self, rhs: R) -> Self::Output {
        Self::Output {
            x: self.x - rhs.borrow().x,
            y: self.y - rhs.borrow().y,
            z: self.z - rhs.borrow().z,
        }
    }
}

impl<R> std::ops::SubAssign<R> for Vector3D
where
    R: Borrow<Vector3D>,
{
    fn sub_assign(&mut self, rhs: R) {
        self.x -= rhs.borrow().x;
        self.y -= rhs.borrow().y;
        self.z -= rhs.borrow().z;
    }
}

impl std::ops::Mul<&Vector3D> for &Vector3D {
    type Output = f64;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<'a> std::ops::Div<f64> for &'a Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vector3D {
    pub fn f2_norm(&self) -> f64 {
        self * self
    }

    pub fn len(&self) -> f64 {
        self.f2_norm().sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.len()
    }

    pub fn cross<R>(&self, rhs: R) -> Self
    where
        R: Borrow<Vector3D>,
    {
        Self {
            x: self.y * rhs.borrow().z - self.z * rhs.borrow().y,
            y: self.z * rhs.borrow().x - self.x * rhs.borrow().z,
            z: self.x * rhs.borrow().y - self.y * rhs.borrow().z,
        }
    }
}
