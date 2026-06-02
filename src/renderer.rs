use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;


const MAX_DEPTH: i32 = 5;

// Trace a ray recursively through the scene to calculate color
pub fn trace_ray(ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
    // 1. If we reach the recursion limit, stop and return black
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut closest_t = f64::INFINITY;
    let mut closest_hit = None;

    // 2. Loop through all objects to find the closest collision
    for obj in &scene.objects {
        if let Some(hit) = obj.intersect(ray) {
            if hit.t < closest_t {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
    }

    if let Some(hit) = closest_hit {
        // Retrieve surface texture color at the hit point
        let material_color = hit.material.texture.color_at(hit.p);

        // Base ambient component
        let ambient = hit.material.ambient * scene.ambient_light;

        let mut diffuse_sum = Vec3::zero();
        let mut specular_sum = Vec3::zero();

        // 3. Loop through all lights to calculate diffuse and specular illumination
        for light in &scene.lights {
            let to_light = light.position - hit.p;
            let dist_to_light = to_light.length();
            let light_dir = to_light.normalize();

            // Cast a shadow ray from the hit point towards the light source
            // Bias the start point slightly along the normal to prevent self-shadowing!
            let shadow_ray = Ray::new(hit.p + hit.normal * 1e-4, light_dir);
            let mut in_shadow = false;

            // Check if any shape blocks this light source
            for obj in &scene.objects {
                if let Some(shadow_hit) = obj.intersect(&shadow_ray) {
                    if shadow_hit.t < dist_to_light {
                        in_shadow = true;
                        break;
                    }
                }
            }

            // 4. If not in shadow, calculate Lambertian diffuse and Blinn-Phong specular
            if !in_shadow {
                // Diffuse
                let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
                diffuse_sum += light.color * (hit.material.diffuse * n_dot_l * light.intensity);

                // Specular
                let view_dir = -ray.direction;
                let reflect_dir = (-light_dir).reflect(&hit.normal).normalize();
                let r_dot_v = reflect_dir.dot(&view_dir).max(0.0);
                let spec_factor = r_dot_v.powf(hit.material.shininess);
                specular_sum +=
                    light.color * (hit.material.specular * spec_factor * light.intensity);
            }
        }

        // Combine local direct lighting components
        let local_color = material_color * (ambient + diffuse_sum) + specular_sum;

        // 5. Recursive Reflection (Mirror surfaces)
        let mut reflected_color = Vec3::zero();
        if hit.material.reflective > 0.0 {
            let reflect_dir = ray.direction.reflect(&hit.normal).normalize();
            let reflect_ray = Ray::new(hit.p + hit.normal * 1e-4, reflect_dir);
            reflected_color = trace_ray(&reflect_ray, scene, depth - 1);
        }

        // 6. Recursive Refraction & Transparency (Glass/Water surfaces)
        let mut refracted_color = Vec3::zero();
        let mut is_refracted = false;
        let mut cos_theta = 0.0;

        if hit.material.transparency > 0.0 {
            let dot_product = ray.direction.dot(&hit.normal);

            // Determine if the ray is entering or leaving the object
            let (normal_out, etai_over_etat, c_theta) = if dot_product < 0.0 {
                (
                    hit.normal,
                    1.0 / hit.material.refractive_index,
                    -dot_product,
                ) // Entering shape
            } else {
                (-hit.normal, hit.material.refractive_index, dot_product) // Leaving shape
            };
            cos_theta = c_theta;

            if let Some(refract_dir) = ray.direction.refract(&normal_out, etai_over_etat) {
                // Bias slightly along inside normal direction to prevent self-intersection inside glass
                let refract_ray = Ray::new(hit.p - normal_out * 1e-4, refract_dir);
                refracted_color = trace_ray(&refract_ray, scene, depth - 1);
                is_refracted = true;
            }
        }

        // 7. Blend shading components based on materials
        if hit.material.transparency > 0.0 && is_refracted {
            // Apply Fresnel's Schlick Approximation for realistic glass borders
            let r0 = ((1.0 - hit.material.refractive_index)
                / (1.0 + hit.material.refractive_index))
                .powi(2);
            let fresnel = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

            let glass_blend = reflected_color * fresnel + refracted_color * (1.0 - fresnel);
            Vec3::lerp(local_color, glass_blend, hit.material.transparency)
        } else if hit.material.reflective > 0.0 {
            local_color * (1.0 - hit.material.reflective)
                + reflected_color * hit.material.reflective
        } else {
            local_color
        }
    } else {
        // 8. If the ray misses all shapes, render a subtle dark gradient sky background
        let t = 0.5 * (ray.direction.y + 1.0);
        Vec3::lerp(Vec3::new(0.01, 0.02, 0.05), Vec3::new(0.08, 0.12, 0.22), t)
    }
}

// Renders a full scene using highly optimized parallel multi-threading
pub fn render(scene: &Scene, width: usize, height: usize) -> Vec3s {
    let mut pixels = vec![Vec3::zero(); width * height];
    
    // Determine the number of CPU threads available on your computer
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let rows_per_thread = (height + num_threads - 1) / num_threads;

    // Use standard scoped threads to borrow `scene` from the stack safely
    std::thread::scope(|scope| {
        let mut threads = vec![];

        for t_idx in 0..num_threads {
            let start_y = t_idx * rows_per_thread;
            let end_y = (start_y + rows_per_thread).min(height);
            if start_y >= height {
                break;
            }

            threads.push(scope.spawn(move || {
                let mut rows = Vec::with_capacity(width * (end_y - start_y));

                for y in start_y..end_y {
                    for x in 0..width {
                        // Normalize screen space coordinates to [0.0, 1.0]
                        let s = x as f64 / (width - 1) as f64;
                        // Flip Y so that coordinate y = 0 sits at the top of the output image
                        let t = (height - 1 - y) as f64 / (height - 1) as f64;

                        let ray = scene.camera.get_ray(s, t);
                        let color = trace_ray(&ray, scene, MAX_DEPTH);
                        rows.push(color);
                    }
                }
                (start_y, rows)
            }));
        }

        // Wait for all threads to complete and stitch the image rows together
        for handle in threads {
            if let Ok((start_y, rows)) = handle.join() {
                let start_idx = start_y * width;
                for (idx, color) in rows.into_iter().enumerate() {
                    pixels[start_idx + idx] = color;
                }
            }
        }
    });

    pixels
}

// Simple type alias to represent list of colors
pub type Vec3s = Vec<Vec3>;

