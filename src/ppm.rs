use crate::vec3::Vec3;

// Formats pixel RGB vectors into a clean, standard P3 ASCII PPM string
pub fn write_ppm(pixels: &[Vec3], width: usize, height: usize) -> String {
    // 1. Pre-allocate string memory capacity to make execution fast
    let mut output = String::with_capacity(32 + pixels.len() * 12);

    // 2. Write the PPM P3 image header
    output.push_str("P3\n");
    output.push_str(&format!("{} {}\n", width, height));
    output.push_str("255\n");

    // 3. Loop through all pixels, format colors, and write to the output string
    for pixel in pixels {
        // Apply clamp to range [0.0, 1.0] and gamma-2 correction (square root)
        let r = pixel.x.clamp(0.0, 1.0).sqrt();
        let g = pixel.y.clamp(0.0, 1.0).sqrt();
        let b = pixel.z.clamp(0.0, 1.0).sqrt();

        let ir = (255.999 * r) as i32;
        let ig = (255.999 * g) as i32;
        let ib = (255.999 * b) as i32;

        output.push_str(&format!("{} {} {}\n", ir, ig, ib));
    }

    output
}
