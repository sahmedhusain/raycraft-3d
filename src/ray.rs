use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    // Build a new Ray
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(), // ALWAYS a unit vector
        }
    }
    // Find the 3D position along the ray at distance 't'
    #[inline]
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
