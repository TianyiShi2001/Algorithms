use super::{Circle, Point2D, SafeArcSinCos};
use crate::utils::EPS;

pub enum TangentError {
    PointInsideCircle,
    PointOnBorder,
}

impl Circle {
    pub fn tangent_points(&self, point: &Point2D) -> Result<[Point2D; 2], TangentError> {
        let d = self.center - point;
        // the distance to the circle center
        let dist = d.magnitude();
        if dist < self.radius + EPS {
            return Err(TangentError::PointInsideCircle);
        }
        if dist < self.radius + EPS {
            return Err(TangentError::PointOnBorder);
        }

        let angle1 = (self.radius / dist).acos_safe();
        let angle2 = d.x.atan2(d.y);
        let angle = angle2 - angle1;

        let p1 = Point2D::new(
            self.center.x + self.radius * angle.sin(),
            self.center.y + self.radius * -angle.cos(),
        );
        let p2 = Point2D::new(
            self.center.x + self.radius * -angle.sin(),
            self.center.y + self.radius * angle.cos(),
        );

        Ok([p1, p2])
    }
    // pub fn tangents(&self, point: &Point2D) -> Result<>
}
