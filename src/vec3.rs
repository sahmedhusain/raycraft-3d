use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // custom vector
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // zero vector (0, 0, 0)
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // unit vector (1, 1, 1)
    #[inline]
    pub const fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    // Dot Product
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross Product
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Squared Length of the vector
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    // Length of the vector
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // length equal to 1.0
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            *self / len
        }
    }

    // Reflection (Mirror effects)
    #[inline]
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * (2.0 * self.dot(normal))
    }

    // Refraction
    #[inline]
    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Option<Self> {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel_len_sq = 1.0 - r_out_perp.length_squared();
        if r_out_parallel_len_sq < 0.0 {
            None // Total Internal Reflection occurs (ray cannot refract, must reflect)
        } else {
            let r_out_parallel = *normal * -r_out_parallel_len_sq.sqrt();
            Some(r_out_perp + r_out_parallel)
        }
    }

    // Interpolation
    #[inline]
    pub fn lerp(a: Self, b: Self, t: f64) -> Self {
        a * (1.0 - t) + b * t
    }
}

// Vector Addition
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// Vector Subtraction
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
// Vector * Scalar Multiplication
impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Scalar * Vector Multiplication
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

// Vector / Scalar Division
impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Vector * Vector Element-wise Multiplication
impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// Unary Negation -vec
impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
