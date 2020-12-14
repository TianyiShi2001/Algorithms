pub mod angle_between_vectors;
pub mod tangent;

use std::ops::Sub;

pub const EPS: f64 = 1e-6;

pub struct Vec2D {
    x: f64,
    y: f64,
}

pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Computes the dot product with another 2D vector.
    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    /// Computes the dot product with another 3D vector.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Copy, Clone)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Sub<&Self> for Point2D {
    type Output = Vec2D;
    fn sub(self, rhs: &Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance_to_point(&self, other: &Self) -> f64 {
        Vec2D::new(self.x - other.x, self.y - other.y).magnitude()
    }
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn distance_to_point(&self, other: &Self) -> f64 {
        Vec3D::new(self.x - other.x, self.y - other.y, self.z - other.z).magnitude()
    }
}

pub struct Circle {
    center: Point2D,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point2D { x, y },
            radius,
        }
    }
}

use num_traits::Float;
pub trait SafeArcSinCos: Float {
    // Due to double rounding precision the value passed into the asin
    // function may be outside its domain of [-1, +1] which would return
    // the value NaN which we do not want.
    fn asin_safe(self) -> Self;
    // Due to double rounding precision the value passed into the acos
    // function may be outside its domain of [-1, +1] which would return
    // the value NaN which we do not want.
    fn acos_safe(self) -> Self;
}

impl SafeArcSinCos for f64 {
    fn asin_safe(self) -> Self {
        if self <= -1.0 {
            -std::f64::consts::PI / 2.0
        } else if self >= 1.0 {
            std::f64::consts::PI / 2.0
        } else {
            self.asin()
        }
    }
    fn acos_safe(self) -> Self {
        if self <= -1.0 {
            std::f64::consts::PI
        } else if self >= 1.0 {
            0.
        } else {
            self.acos()
        }
    }
}
