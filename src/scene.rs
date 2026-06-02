use crate::camera::Camera;
use crate::light::Light;
use crate::material::{Material, Texture};
use crate::object::{Cube, Cylinder, Intersect, Plane, Sphere};
use crate::vec3::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Scene {
    pub objects: Vec<Box<dyn Intersect>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub ambient_light: Vec3,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Intersect>>,
        lights: Vec<Light>,
        camera: Camera,
        ambient_light: Vec3,
    ) -> Self {
        Self {
            objects,
            lights,
            camera,
            ambient_light,
        }
    }

    // Built-in Scene 1: Single Sphere (Standard Brightness)
    pub fn build_scene_1(aspect_ratio: f64) -> Self {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
        );

        let red_shiny = Material::shiny(Vec3::new(0.9, 0.1, 0.15), 0.4);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.5, red_shiny);

        let lights = vec![
            Light::new(Vec3::new(5.0, 5.0, -2.0), 1.5, Vec3::new(1.0, 1.0, 1.0)), // Bright Light
        ];

        Self {
            objects: vec![Box::new(sphere)],
            lights,
            camera,
            ambient_light: Vec3::new(0.15, 0.15, 0.15),
        }
    }

    // Built-in Scene 2: Plane & Low-Brightness Cube
    pub fn build_scene_2(aspect_ratio: f64) -> Self {
        let camera = Camera::new(
            Vec3::new(0.0, 0.8, 1.0),
            Vec3::new(0.0, -0.2, -4.0),
            Vec3::new(0.0, 1.0, 0.0),
            45.0,
            aspect_ratio,
        );

        let plane_mat = Material::checker(Vec3::new(0.4, 0.4, 0.4), Vec3::new(0.8, 0.8, 0.8), 2.0);
        let plane = Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            plane_mat,
        );

        let cube_mat = Material::new(
            Texture::Solid(Vec3::new(0.1, 0.6, 0.6)),
            0.05,
            0.5,
            0.1,
            10.0,
            0.0,
            1.0,
            0.0,
        );
        let cube = Cube::new(
            Vec3::new(-0.6, -1.0, -4.5),
            Vec3::new(0.6, 0.2, -3.3),
            cube_mat,
        );

        let lights = vec![Light::new(
            Vec3::new(3.0, 4.0, -2.0),
            0.4,
            Vec3::new(1.0, 1.0, 1.0),
        )];

        Self {
            objects: vec![Box::new(plane), Box::new(cube)],
            lights,
            camera,
            ambient_light: Vec3::new(0.05, 0.05, 0.05),
        }
    }

    // Built-in Scene 3: Showroom with all objects
    pub fn build_scene_3(aspect_ratio: f64) -> Self {
        let camera = Camera::new(
            Vec3::new(0.0, 1.0, 1.5),
            Vec3::new(0.0, -0.1, -4.0),
            Vec3::new(0.0, 1.0, 0.0),
            50.0,
            aspect_ratio,
        );

        Self::construct_scene_3_and_4_objects(camera)
    }

    // Built-in Scene 4: Same showroom, alternate camera perspective
    pub fn build_scene_4(aspect_ratio: f64) -> Self {
        let camera = Camera::new(
            Vec3::new(2.8, 2.0, -1.0),
            Vec3::new(0.0, -0.1, -4.0),
            Vec3::new(0.0, 1.0, 0.0),
            50.0,
            aspect_ratio,
        );

        Self::construct_scene_3_and_4_objects(camera)
    }

    fn construct_scene_3_and_4_objects(camera: Camera) -> Self {
        let mut objects: Vec<Box<dyn Intersect>> = Vec::new();

        // 1. Flat Plane (Checkerboard Floor)
        let plane_mat = Material::checker(Vec3::new(0.3, 0.3, 0.3), Vec3::new(0.7, 0.7, 0.7), 1.5);
        let plane = Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            plane_mat,
        );
        objects.push(Box::new(plane));

        // 2. Sphere (Shiny Blue, 30% reflective)
        let sphere_mat = Material::shiny(Vec3::new(0.1, 0.3, 0.9), 0.3);
        let sphere = Sphere::new(Vec3::new(-1.6, -0.2, -4.2), 0.8, sphere_mat);
        objects.push(Box::new(sphere));

        // 3. Cube (Glossy Green, 15% reflective)
        let cube_mat = Material::new(
            Texture::Solid(Vec3::new(0.1, 0.8, 0.2)),
            0.1,
            0.7,
            0.3,
            40.0,
            0.15,
            1.0,
            0.0,
        );
        let cube = Cube::new(
            Vec3::new(0.8, -1.0, -3.8),
            Vec3::new(1.8, 0.0, -2.8),
            cube_mat,
        );
        objects.push(Box::new(cube));

        // 4. Cylinder (Shiny Orange)
        let cyl_mat = Material::shiny(Vec3::new(0.9, 0.45, 0.05), 0.2);
        let cylinder = Cylinder::new(Vec3::new(-0.3, -1.0, -3.5), 0.4, 1.2, cyl_mat);
        objects.push(Box::new(cylinder));

        // 5. Glass Sphere (Refractive & Transparent, situated at center-back)
        let glass_mat = Material::glass(1.5, 0.9);
        let glass_sphere = Sphere::new(Vec3::new(-0.2, 0.5, -5.0), 0.6, glass_mat);
        objects.push(Box::new(glass_sphere));

        // Two light sources (Primary key light and a secondary soft blue fill light)
        let lights = vec![
            Light::new(Vec3::new(4.0, 6.0, -1.0), 1.5, Vec3::new(1.0, 1.0, 1.0)),
            Light::new(Vec3::new(-3.0, 3.0, -2.0), 0.5, Vec3::new(0.8, 0.8, 0.9)),
        ];

        Self {
            objects,
            lights,
            camera,
            ambient_light: Vec3::new(0.15, 0.15, 0.18),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P, aspect_ratio: f64) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);

        let mut objects: Vec<Box<dyn Intersect>> = Vec::new();
        let mut lights: Vec<Light> = Vec::new();

        let mut camera_pos = Vec3::new(0.0, 0.0, 0.0);
        let mut camera_look = Vec3::new(0.0, 0.0, -5.0);
        let camera_up = Vec3::new(0.0, 1.0, 0.0);
        let mut camera_fov = 40.0;
        let mut ambient_light = Vec3::new(0.1, 0.1, 0.1);

        for (idx, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Line error: {}", e))?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            let parse_f64 = |t: &str, name: &str| -> Result<f64, String> {
                t.parse::<f64>()
                    .map_err(|_| format!("Line {}: invalid float for field '{}'", idx + 1, name))
            };

            let parse_vec3 = |t: &[&str], name: &str| -> Result<Vec3, String> {
                if t.len() < 3 {
                    return Err(format!(
                        "Line {}: missing Vec3 fields for '{}'",
                        idx + 1,
                        name
                    ));
                }
                let x = parse_f64(t[0], name)?;
                let y = parse_f64(t[1], name)?;
                let z = parse_f64(t[2], name)?;
                Ok(Vec3::new(x, y, z))
            };

            match tokens[0] {
                "camera" => {
                    if tokens.len() < 8 {
                        return Err(format!(
                            "Line {}: camera requires 7 values (pos_x pos_y pos_z look_x look_y look_z fov)",
                            idx + 1
                        ));
                    }
                    camera_pos = parse_vec3(&tokens[1..4], "camera position")?;
                    camera_look = parse_vec3(&tokens[4..7], "camera look_at")?;
                    camera_fov = parse_f64(tokens[7], "camera fov")?;
                }
                "ambient" => {
                    if tokens.len() < 4 {
                        return Err(format!(
                            "Line {}: ambient requires 3 values (r g b)",
                            idx + 1
                        ));
                    }
                    ambient_light = parse_vec3(&tokens[1..4], "ambient light")?;
                }
                "light" => {
                    if tokens.len() < 8 {
                        return Err(format!(
                            "Line {}: light requires 7 values (pos_x pos_y pos_z intensity r g b)",
                            idx + 1
                        ));
                    }
                    let pos = parse_vec3(&tokens[1..4], "light position")?;
                    let intensity = parse_f64(tokens[4], "light intensity")?;
                    let color = parse_vec3(&tokens[5..8], "light color")?;
                    lights.push(Light::new(pos, intensity, color));
                }
                "sphere" => {
                    if tokens.len() < 12 {
                        return Err(format!(
                            "Line {}: sphere requires 11 values (cx cy cz radius r g b specular reflective refr_idx transp)",
                            idx + 1
                        ));
                    }
                    let center = parse_vec3(&tokens[1..4], "sphere center")?;
                    let radius = parse_f64(tokens[4], "sphere radius")?;
                    let color = parse_vec3(&tokens[5..8], "sphere color")?;
                    let spec = parse_f64(tokens[8], "sphere specular")?;
                    let refl = parse_f64(tokens[9], "sphere reflective")?;
                    let refr = parse_f64(tokens[10], "sphere refractive index")?;
                    let transp = parse_f64(tokens[11], "sphere transparency")?;

                    let mat = Material::new(
                        Texture::Solid(color),
                        0.1,
                        0.7,
                        spec,
                        50.0,
                        refl,
                        refr,
                        transp,
                    );
                    objects.push(Box::new(Sphere::new(center, radius, mat)));
                }
                "plane" => {
                    if tokens.len() < 12 {
                        return Err(format!(
                            "Line {}: plane requires at least 11 values",
                            idx + 1
                        ));
                    }
                    let pt = parse_vec3(&tokens[1..4], "plane point")?;
                    let n = parse_vec3(&tokens[4..7], "plane normal")?;
                    let color = parse_vec3(&tokens[7..10], "plane color")?;
                    let spec = parse_f64(tokens[10], "plane specular")?;
                    let refl = parse_f64(tokens[11], "plane reflective")?;

                    // Check if optional checkerboard parameters are present
                    let texture = if tokens.len() >= 16 {
                        let scale = parse_f64(tokens[12], "checker scale")?;
                        let c2 = parse_vec3(&tokens[13..16], "checker color 2")?;
                        Texture::Checker(color, c2, scale)
                    } else {
                        Texture::Solid(color)
                    };

                    let mat = Material::new(texture, 0.1, 0.8, spec, 20.0, refl, 1.0, 0.0);
                    objects.push(Box::new(Plane::new(pt, n, mat)));
                }
                "cube" => {
                    if tokens.len() < 12 {
                        return Err(format!("Line {}: cube requires 11 values", idx + 1));
                    }
                    let min_pt = parse_vec3(&tokens[1..4], "cube min")?;
                    let max_pt = parse_vec3(&tokens[4..7], "cube max")?;
                    let color = parse_vec3(&tokens[7..10], "cube color")?;
                    let spec = parse_f64(tokens[10], "cube specular")?;
                    let refl = parse_f64(tokens[11], "cube reflective")?;

                    let mat =
                        Material::new(Texture::Solid(color), 0.1, 0.8, spec, 30.0, refl, 1.0, 0.0);
                    objects.push(Box::new(Cube::new(min_pt, max_pt, mat)));
                }
                "cylinder" => {
                    if tokens.len() < 11 {
                        return Err(format!("Line {}: cylinder requires 10 values", idx + 1));
                    }
                    let center = parse_vec3(&tokens[1..4], "cylinder center")?;
                    let radius = parse_f64(tokens[4], "cylinder radius")?;
                    let height = parse_f64(tokens[5], "cylinder height")?;
                    let color = parse_vec3(&tokens[6..9], "cylinder color")?;
                    let spec = parse_f64(tokens[9], "cylinder specular")?;
                    let refl = parse_f64(tokens[10], "cylinder reflective")?;

                    let mat =
                        Material::new(Texture::Solid(color), 0.1, 0.8, spec, 30.0, refl, 1.0, 0.0);
                    objects.push(Box::new(Cylinder::new(center, radius, height, mat)));
                }
                _ => {
                    return Err(format!("Line {}: Unknown command '{}'", idx + 1, tokens[0]));
                }
            }
        }

        if lights.is_empty() {
            lights.push(Light::new(
                Vec3::new(0.0, 5.0, 0.0),
                1.0,
                Vec3::new(1.0, 1.0, 1.0),
            ));
        }

        let camera = Camera::new(camera_pos, camera_look, camera_up, camera_fov, aspect_ratio);

        Ok(Self::new(objects, lights, camera, ambient_light))
    }
}
