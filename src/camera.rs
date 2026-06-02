use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov: f64,

    // Calculated coordinate
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // Constructor to build and calculate viewport dimensions
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect_ratio: f64) -> Self {
        // 1. Convert FOV from degrees to radians
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // 2. Compute camera coordinate framework vectors (orthogonal basis)
        let w = (position - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        // 3. Define the physical viewport vectors in 3D space
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        // 4. Locate the coordinate of the viewport's lower-left corner
        let lower_left_corner = position - horizontal * 0.5 - vertical * 0.5 - w;

        Self {
            position,
            look_at,
            up,
            fov,
            u,
            v,
            w,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    // Generate a Ray pointing from the camera lens through a specific pixel coordinate (s, t)
    #[inline]
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // 1. Calculate the 3D position on the viewport grid
        let viewport_point = self.lower_left_corner + self.horizontal * s + self.vertical * t;

        // 2. Compute the direction from the camera origin to that point
        let direction = viewport_point - self.position;

        // 3. Return the new Ray
        Ray::new(self.position, direction)
    }
}
