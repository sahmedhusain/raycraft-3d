use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Solid(Vec3),
    Checker(Vec3, Vec3, f64),
}

impl Texture {
    #[inline]
    pub fn color_at(&self, u: f64, v: f64, enable_textures: bool) -> Vec3 {
        match self {
            Texture::Solid(c) => *c,
            Texture::Checker(c1, c2, scale) => {
                if !enable_textures {
                    return *c1; // Fall back to solid color when textures are disabled
                }
                // Alternate grid squares based on UV coordinate sums
                let u_grid = (u * scale).floor() as i32;
                let v_grid = (v * scale).floor() as i32;
                if (u_grid + v_grid) % 2 == 0 {
                    *c1
                } else {
                    *c2
                }
            }
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    // 1. Matte/Rough Material preset (e.g. chalk, rubber)
    pub fn matte(color: Vec3) -> Self {
        Self {
            texture: Texture::Solid(color),
            ambient: 0.1,
            diffuse: 0.8,
            specular: 0.1,
            shininess: 10.0,
            reflective: 0.0,
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    // 2. Shiny/Glossy Material preset (e.g. plastic, painted sphere)
    pub fn shiny(color: Vec3, reflective: f64) -> Self {
        Self {
            texture: Texture::Solid(color),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.4,
            shininess: 75.0,
            reflective,
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    // 3. Mirror Material preset (highly reflective)
    pub fn mirror() -> Self {
        Self {
            texture: Texture::Solid(Vec3::new(0.95, 0.95, 0.95)),
            ambient: 0.05,
            diffuse: 0.1,
            specular: 0.9,
            shininess: 150.0,
            reflective: 0.9,
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    // 4. Glass/Water Material preset (refractive & transparent)
    pub fn glass(refractive_index: f64, transparency: f64) -> Self {
        Self {
            texture: Texture::Solid(Vec3::new(0.98, 0.98, 0.98)),
            ambient: 0.0,
            diffuse: 0.05,
            specular: 0.95,
            shininess: 200.0,
            reflective: 0.2,
            refractive_index,
            transparency,
        }
    }

    // 5. Checkerboard Material preset (ideal for floors)
    pub fn checker(c1: Vec3, c2: Vec3, scale: f64) -> Self {
        Self {
            texture: Texture::Checker(c1, c2, scale),
            ambient: 0.2,
            diffuse: 0.8,
            specular: 0.0,
            shininess: 10.0,
            reflective: 0.0,
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }
}
