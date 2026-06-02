use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub color: Vec3,
}

impl Light {
    // 1. Constructor for a light source with custom color
    pub fn new(position: Vec3, intensity: f64, color: Vec3) -> Self {
        Self {
            position,
            intensity,
            color,
        }
    }

    // 2. Helper constructor for standard white light
    pub fn white(position: Vec3, intensity: f64) -> Self {
        Self {
            position,
            intensity,
            color: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}
