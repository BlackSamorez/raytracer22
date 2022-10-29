use num_traits::{Zero, Float};

pub trait Sqrt : Float {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

pub trait ISqrt : Sqrt {
    fn isqrt(&self) -> Self {
        Self::one() / Sqrt::sqrt(self)
    }
}

impl ISqrt for f64 {}

impl ISqrt for f32 {
    fn isqrt(&self) -> Self {
        let i = self.to_bits();
        let i = 0x5f3759df - (i >> 1);
        let y = f32::from_bits(i);

        y * (1.5 - 0.5 * self * y * y)
    }
}

pub trait Vectorizable : Float + Zero + Default + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + ISqrt {}
impl Vectorizable for f32 {}
impl Vectorizable for f64 {}

#[derive(Default, Eq, PartialEq)]
struct Vector3D<T: Vectorizable> {
    x: T,
    y: T,
    z: T,
}

impl<T: Vectorizable> Vector3D<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, 'b, T: Vectorizable> std::ops::Add<&'b Vector3D<T>> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    fn add(self, rhs: &Vector3D<T>) -> Self::Output {
        Self::Output{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T: Vectorizable> std::ops::AddAssign<&Self> for Vector3D<T> {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<'a, 'b, T: Vectorizable> std::ops::Sub<&'b Vector3D<T>> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    fn sub(self, rhs: &Vector3D<T>) -> Self::Output {
        Self::Output{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T: Vectorizable> std::ops::SubAssign<&Self> for Vector3D<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<'a, 'b, T: Vectorizable> std::ops::Mul<&'b Vector3D<T>> for &'a Vector3D<T> {
    type Output = T;

    fn mul(self, rhs: &Vector3D<T>) -> Self::Output {
       self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Vectorizable> std::ops::Mul<T> for Vector3D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<T: Vectorizable> std::ops::MulAssign<T> for Vector3D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<'a, T: Vectorizable> std::ops::Div<T> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3D{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl<T: Vectorizable> std::ops::DivAssign<T> for Vector3D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Vectorizable> Vector3D<T> {
    pub fn f2_norm(&self) -> T {
        self * self
    }

    pub fn normalize(&mut self) {
        *self *= self.f2_norm().isqrt()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self{x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.z}
    }
}
