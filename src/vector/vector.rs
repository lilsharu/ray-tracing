use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3d = Vector3d;

impl Default for Vector3d {
    fn default() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }
}

impl Add for Vector3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl SubAssign for Vector3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vector3d {
    type Output = Self;

    fn mul(self, scale_factor: f64) -> Self {
        Self {
            x: self.x * scale_factor,
            y: self.y * scale_factor,
            z: self.z * scale_factor,
        }
    }
}

impl MulAssign<f64> for Vector3d {
    fn mul_assign(&mut self, scale_factor: f64) {
        self.x *= scale_factor;
        self.y *= scale_factor;
        self.z *= scale_factor;
    }
}

impl Div<f64> for Vector3d {
    type Output = Self;

    fn div(self, scale_factor: f64) -> Self {
        Self {
            x: self.x / scale_factor,
            y: self.y / scale_factor,
            z: self.z / scale_factor,
        }
    }
}

impl DivAssign<f64> for Vector3d {
    fn div_assign(&mut self, scale_factor: f64) {
        self.x /= scale_factor;
        self.y /= scale_factor;
        self.z /= scale_factor;
    }
}

impl Display for Vector3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {}, {})", self.x, self.y, self.z).as_str())
    }
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// The dot product of the vector and @rhs
    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.x + self.z * rhs.z
    }

    /// The cross product of the vector and @rhs
    fn cross(&self, rhs: &Self) -> Vector3d {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// The square of the magnitude of the vector
    ///
    /// Implemented as `self.dot(self)`
    fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    /// The magnitude of the vector
    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn unit(&self) -> Self {
        *self / self.magnitude()
    }
    
    fn normalize(&mut self) {
        *self /= self.magnitude();
    }
}
