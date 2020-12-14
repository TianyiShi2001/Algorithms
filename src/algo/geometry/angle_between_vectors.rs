use super::{SafeArcSinCos, Vec2D, Vec3D};

impl Vec2D {
    /// Return the smaller of the two angles between two 2D vectors in radians
    pub fn angle(&self, other: &Self) -> f64 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos_safe()
    }
}

impl Vec3D {
    /// Return the smaller of the two angles between two 3D vectors in radians
    pub fn angle(&self, other: &Self) -> f64 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos_safe()
    }
}
