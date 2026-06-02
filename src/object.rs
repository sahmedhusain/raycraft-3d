use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

// Stores details about a ray-shape collision
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,              // Distance from ray origin to collision point
    pub p: Vec3,             // Exact 3D point of collision
    pub normal: Vec3,        // Normal vector pointing perpendicular to the surface
    pub u: f64,              // Texture coordinate U in [0.0, 1.0]
    pub v: f64,              // Texture coordinate V in [0.0, 1.0]
    pub material: Material,  // Material of the object that was hit
}

// Interface that all shapes must implement
pub trait Intersect: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord>;
}

// ----------------------------------------------------
// SPHERE
// ----------------------------------------------------
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // 1. If discriminant is negative, the ray misses the sphere
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // 2. Find the nearest root (closest collision distance) in the positive range
        let mut root = (-half_b - sqrtd) / a;
        if root < 0.0001 {
            root = (-half_b + sqrtd) / a;
            if root < 0.0001 {
                return None;
            }
        }

        // 3. Compute collision position and normal vector
        let p = ray.point_at(root);
        let normal = (p - self.center) / self.radius;

        // 4. Orient normal towards the incoming ray (important for transparent/inside hits)
        let normal = if ray.direction.dot(&normal) < 0.0 {
            normal
        } else {
            -normal
        };

        // Calculate spherical UV texture coordinates
        let d = (p - self.center).normalize();
        let phi = d.z.atan2(d.x);
        let theta = d.y.asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI;

        Some(HitRecord {
            t: root,
            p,
            normal,
            u,
            v,
            material: self.material,
        })
    }
}

// ----------------------------------------------------
// PLANE
// ----------------------------------------------------
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        // 1. Calculate the dot product between the plane normal and ray direction
        let denom = self.normal.dot(&ray.direction);

        // 2. If the denominator is close to 0, the ray is parallel to the plane
        if denom.abs() < 1e-6 {
            return None;
        }

        // 3. Solve for 't' (distance)
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        if t < 0.0001 {
            return None;
        }

        // 4. Compute hit position and orient the normal towards the ray
        let p = ray.point_at(t);
        let normal = if denom < 0.0 {
            self.normal
        } else {
            -self.normal
        };

        // Calculate planar UV texture coordinates based on plane orientation
        let (u, v) = if self.normal.y.abs() > 0.8 {
            (p.x, p.z)
        } else if self.normal.x.abs() > 0.8 {
            (p.y, p.z)
        } else {
            (p.x, p.y)
        };

        Some(HitRecord {
            t,
            p,
            normal,
            u,
            v,
            material: self.material,
        })
    }
}

// ----------------------------------------------------
// CUBE (Axis-Aligned Bounding Box - AABB)
// ----------------------------------------------------
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Self { min, max, material }
    }
}

impl Intersect for Cube {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let mut t_min = -f64::INFINITY;
        let mut t_max = f64::INFINITY;

        // 1. Check intersection intervals for X axis
        let tx1 = (self.min.x - ray.origin.x) / ray.direction.x;
        let tx2 = (self.max.x - ray.origin.x) / ray.direction.x;
        t_min = t_min.max(tx1.min(tx2));
        t_max = t_max.min(tx1.max(tx2));

        // 2. Check intersection intervals for Y axis
        let ty1 = (self.min.y - ray.origin.y) / ray.direction.y;
        let ty2 = (self.max.y - ray.origin.y) / ray.direction.y;
        t_min = t_min.max(ty1.min(ty2));
        t_max = t_max.min(ty1.max(ty2));

        // 3. Check intersection intervals for Z axis
        let tz1 = (self.min.z - ray.origin.z) / ray.direction.z;
        let tz2 = (self.max.z - ray.origin.z) / ray.direction.z;
        t_min = t_min.max(tz1.min(tz2));
        t_max = t_max.min(tz1.max(tz2));

        // 4. If intervals don't overlap, the ray misses the box
        if t_max < t_min || t_max < 0.0001 {
            return None;
        }

        let t = if t_min > 0.0001 { t_min } else { t_max };
        if t < 0.0001 {
            return None;
        }

        let p = ray.point_at(t);

        // 5. Calculate which face of the cube was hit to set the normal
        let normal;
        let bias = 1e-4;

        if (p.x - self.min.x).abs() < bias {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (p.x - self.max.x).abs() < bias {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (p.y - self.min.y).abs() < bias {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (p.y - self.max.y).abs() < bias {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (p.z - self.min.z).abs() < bias {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (p.z - self.max.z).abs() < bias {
            normal = Vec3::new(0.0, 0.0, 1.0);
        } else {
            let center = (self.min + self.max) * 0.5;
            let d = p - center;
            let extents = (self.max - self.min) * 0.5;
            let dx = (d.x / extents.x).abs();
            let dy = (d.y / extents.y).abs();
            let dz = (d.z / extents.z).abs();

            if dx > dy && dx > dz {
                normal = Vec3::new(d.x.signum(), 0.0, 0.0);
            } else if dy > dx && dy > dz {
                normal = Vec3::new(0.0, d.y.signum(), 0.0);
            } else {
                normal = Vec3::new(0.0, 0.0, d.z.signum());
            }
        }

        // 6. Orient normal towards incoming ray
        let normal = if ray.direction.dot(&normal) < 0.0 {
            normal
        } else {
            -normal
        };

        // Calculate cubic UV coordinates based on which face normal was hit
        let (u, v) = if normal.x.abs() > 0.8 {
            (
                (p.y - self.min.y) / (self.max.y - self.min.y).max(1e-6),
                (p.z - self.min.z) / (self.max.z - self.min.z).max(1e-6)
            )
        } else if normal.y.abs() > 0.8 {
            (
                (p.x - self.min.x) / (self.max.x - self.min.x).max(1e-6),
                (p.z - self.min.z) / (self.max.z - self.min.z).max(1e-6)
            )
        } else {
            (
                (p.x - self.min.x) / (self.max.x - self.min.x).max(1e-6),
                (p.y - self.min.y) / (self.max.y - self.min.y).max(1e-6)
            )
        };

        Some(HitRecord {
            t,
            p,
            normal,
            u,
            v,
            material: self.material,
        })
    }
}

// ----------------------------------------------------
// CYLINDER (Y-oriented, finite, capped)
// ----------------------------------------------------
pub struct Cylinder {
    pub center: Vec3, // Center coordinates of the base circular disk
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }
}

impl Intersect for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let mut best_t = f64::INFINITY;
        let mut best_normal = Vec3::zero();

        // 1. Intersect with infinite cylinder wall along the Y-axis
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        if a.abs() > 1e-6 {
            let ox_cx = ray.origin.x - self.center.x;
            let oz_cz = ray.origin.z - self.center.z;
            let b = 2.0 * (ray.direction.x * ox_cx + ray.direction.z * oz_cz);
            let c = ox_cx * ox_cx + oz_cz * oz_cz - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant >= 0.0 {
                let sqrtd = discriminant.sqrt();
                for &t in &[(-b - sqrtd) / (2.0 * a), (-b + sqrtd) / (2.0 * a)] {
                    if t > 0.0001 && t < best_t {
                        let p = ray.point_at(t);
                        if p.y >= self.center.y && p.y <= self.center.y + self.height {
                            best_t = t;
                            best_normal = Vec3::new(p.x - self.center.x, 0.0, p.z - self.center.z)
                                .normalize();
                        }
                    }
                }
            }
        }

        // 2. Intersect with the bottom cap disk (Plane at y = center.y)
        if ray.direction.y.abs() > 1e-6 {
            let t_bot = (self.center.y - ray.origin.y) / ray.direction.y;
            if t_bot > 0.0001 && t_bot < best_t {
                let p = ray.point_at(t_bot);
                let dx = p.x - self.center.x;
                let dz = p.z - self.center.z;
                if dx * dx + dz * dz <= self.radius * self.radius {
                    best_t = t_bot;
                    best_normal = Vec3::new(0.0, -1.0, 0.0);
                }
            }
        }

        // 3. Intersect with the top cap disk (Plane at y = center.y + height)
        if ray.direction.y.abs() > 1e-6 {
            let t_top = (self.center.y + self.height - ray.origin.y) / ray.direction.y;
            if t_top > 0.0001 && t_top < best_t {
                let p = ray.point_at(t_top);
                let dx = p.x - self.center.x;
                let dz = p.z - self.center.z;
                if dx * dx + dz * dz <= self.radius * self.radius {
                    best_t = t_top;
                    best_normal = Vec3::new(0.0, 1.0, 0.0);
                }
            }
        }

        // 4. Return closest valid intersection out of walls, bottom cap, and top cap
        if best_t < f64::INFINITY {
            let p = ray.point_at(best_t);
            let normal = if ray.direction.dot(&best_normal) < 0.0 {
                best_normal
            } else {
                -best_normal
            };

            // Calculate cylindrical UV coordinates
            let (u, v) = if best_normal.y.abs() > 0.8 {
                // Hitting top/bottom caps
                (
                    (p.x - self.center.x) / self.radius,
                    (p.z - self.center.z) / self.radius
                )
            } else {
                // Hitting cylinder tube wall
                let phi = (p.z - self.center.z).atan2(p.x - self.center.x);
                (
                    1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI),
                    (p.y - self.center.y) / self.height
                )
            };

            Some(HitRecord {
                t: best_t,
                p,
                normal,
                u,
                v,
                material: self.material,
            })
        } else {
            None
        }
    }
}
