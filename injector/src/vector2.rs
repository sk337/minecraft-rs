use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Initialize a Vector2
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, vector: &Self) -> f64 {
        self.x * vector.x + self.y * vector.y
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(self.x.max(min.x).min(max.x), self.y.max(min.y).min(max.y))
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
        Self::new(self.x.max(vector.x), self.y.max(vector.y))
    }

    pub fn min(&self, vector: &Self) -> Self {
        Self::new(self.x.min(vector.x), self.y.min(vector.y))
    }

    pub fn reflect(&self, vector: &Self) -> Self {
        *self - *vector * (2.0 * self.dot(vector) / vector.length_squared())
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.x.sqrt(), self.y.sqrt())
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    pub fn to_angle(&self, deg: bool) -> f64 {
        let angle = self.y.atan2(self.x);
        if deg {
            angle.to_degrees()
        } else {
            angle
        }
    }
}

// Implement Add trait for Vector2
impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

// Implement Sub trait for Vector2
impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

// Implement Mul trait for Vector2 with scalar
impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

// Implement Div trait for Vector2 with scalar
impl Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

// Implement Mul trait for Vector2 with another Vector2 (element-wise multiplication)
impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x * other.x, self.y * other.y)
    }
}

// Implement Div trait for Vector2 with another Vector2 (element-wise division)
impl Div for Vector2 {
    type Output = Vector2;

    fn div(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x / other.x, self.y / other.y)
    }
}

// Implement AddAssign trait for Vector2
impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Implement SubAssign trait for Vector2
impl std::ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Implement MulAssign trait for Vector2 with scalar
impl std::ops::MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// Implement DivAssign trait for Vector2 with scalar
impl std::ops::DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

// Implement MulAssign trait for Vector2 with another Vector2 (element-wise multiplication)
impl std::ops::MulAssign for Vector2 {
    fn mul_assign(&mut self, other: Vector2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

// Implement DivAssign trait for Vector2 with another Vector2 (element-wise division)
impl std::ops::DivAssign for Vector2 {
    fn div_assign(&mut self, other: Vector2) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

pub enum ScalarOrVector {
    Scalar(f64),
    Vector(Vector2),
}

impl From<f64> for ScalarOrVector {
    fn from(scalar: f64) -> Self {
        ScalarOrVector::Scalar(scalar)
    }
}

impl From<Vector2> for ScalarOrVector {
    fn from(vector: Vector2) -> Self {
        ScalarOrVector::Vector(vector)
    }
}
