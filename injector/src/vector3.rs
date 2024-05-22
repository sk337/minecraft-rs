use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Initialize a Vector3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, vector: &Self) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(
            self.x.max(min.x).min(max.x),
            self.y.max(min.y).min(max.y),
            self.z.max(min.z).min(max.z),
        )
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    pub fn distance(&self, vector: &Self) -> f64 {
        (*self - *vector).length()
    }

    pub fn distance_squared(&self, vector: &Self) -> f64 {
        (*self - *vector).length_squared()
    }

    pub fn lerp(&self, vector: &Self, t: f64) -> Self {
        *self + (*vector - *self) * t
    }

    pub fn max(&self, vector: &Self) -> Self {
        Self::new(
            self.x.max(vector.x),
            self.y.max(vector.y),
            self.z.max(vector.z),
        )
    }

    pub fn min(&self, vector: &Self) -> Self {
        Self::new(
            self.x.min(vector.x),
            self.y.min(vector.y),
            self.z.min(vector.z),
        )
    }

    pub fn reflect(&self, vector: &Self) -> Self {
        *self - *vector * (2.0 * self.dot(vector) / vector.length_squared())
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    pub fn to_angle(&self, deg: bool) -> (f64, f64) {
        let pitch = (self.y.atan2(self.x)).to_degrees();
        let yaw = (self.z.atan2((self.x.powi(2) + self.y.powi(2)).sqrt())).to_degrees();
        if deg {
            (pitch, yaw)
        } else {
            (pitch.to_radians(), yaw.to_radians())
        }
    }
}

// Implement Add trait for Vector3
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// Implement Sub trait for Vector3
impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Implement Mul trait for Vector3 with scalar
impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Implement Div trait for Vector3 with scalar
impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Implement Mul trait for Vector3 with another Vector3 (element-wise multiplication)
impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// Implement Div trait for Vector3 with another Vector3 (element-wise division)
impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

// Implement AddAssign trait for Vector3
impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Implement SubAssign trait for Vector3
impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Implement MulAssign trait for Vector3 with scalar
impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// Implement DivAssign trait for Vector3 with scalar
impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// Implement MulAssign trait for Vector3 with another Vector3 (element-wise multiplication)
impl std::ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

// Implement DivAssign trait for Vector3 with another Vector3 (element-wise division)
impl std::ops::DivAssign for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

pub enum ScalarOrVector {
    Scalar(f64),
    Vector(Vector3),
}

impl From<f64> for ScalarOrVector {
    fn from(scalar: f64) -> Self {
        ScalarOrVector::Scalar(scalar)
    }
}

impl From<Vector3> for ScalarOrVector {
    fn from(vector: Vector3) -> Self {
        ScalarOrVector::Vector(vector)
    }
}
