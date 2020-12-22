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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;
    #[test]
    fn test_angle_between_vectors() {
        let v = Vec2D::new(3., 4.);
        let w = Vec2D::new(4., -3.);
        assert_eq!(v.angle(&w), FRAC_PI_2);
        assert_eq!(v.angle(&v), 0.);

        let v = Vec3D::new(1., 2., 3.);
        let w = Vec3D::new(2., 3., 4.);
        assert_eq!(v.angle(&w), 0.12186756768575456);
        assert_eq!(v.angle(&v), 0.);
    }
}
