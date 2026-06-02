use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Solid(Vec3),
    Checker(Vec3, Vec3, f64),
}

impl Texture {
    #[inline]
    pub fn color_at(&self, p: Vec3) -> Vec3 {
        match self {
            Texture::Solid(c) => *c,
            Texture::Checker(c1, c2, scale) => {
                let s = scale;
                let s_val =
                    (p.x * s).floor() as i32 + (p.y * s).floor() as i32 + (p.z * s).floor() as i32;
                if s_val % 2 == 0 { *c1 } else { *c2 }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub texture: Texture,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub refractive_index: f64,
    pub transparency: f64,
}

impl Material {
    // Custom constructor
    pub fn new(
        texture: Texture,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        refractive_index: f64,
        transparency: f64,
    ) -> Self {
        Self {
            texture,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            refractive_index,
            transparency,
        }
    }

    