# rt (Ray Tracer in Rust)

A high-performance, concurrent 3D **Ray Tracer (rt)** implemented in Rust from scratch with zero external dependencies. It supports rendering standard geometric shapes, lighting, shadows, and optional recursive reflections and refractions behind performance flags.

---

## Features

1. **Four Core Geometries:** Sphere, Plane, Cube (AABB), and capped Cylinder with fully rotatable normals.
2. **Projective Camera Model:** Fully rotatable and movable camera with custom Field of View (FOV) and automatic viewport mapping.
3. **Advanced Shading & Lighting:** Blinn-Phong specular highlights, Lambertian diffuse lighting, ambient light settings, and ray-traced shadows.
4. **Recursive Reflections & Refractions (Bonus):** Realistic metallic mirrors and refractive glass utilizing Snell's Law and Fresnel's Schlick Approximation (enabled via `-r` flag).
5. **Procedural Textures (Bonus):** Checkerboard texturing for planes and floors (enabled via `-t` flag).
6. **Multi-Threaded Rendering:** Automatic utilization of all available CPU cores to render rows in parallel using standard scoped threads.
7. **Flexible CLI & Resolution Controls:** Custom output dimensions to easily downscale for fast previews and upscale to high-resolution outputs.
8. **Extensible Scene Loader:** Load built-in scenes directly or pass a `.rt` configuration file to dynamically construct custom 3D worlds.

---

## Installation & Usage

Make sure you have [Rust and Cargo](https://rustup.rs/) installed.

### 1. Show Help Menu
To see all available CLI options:
```bash
cargo run -- --help
```

### 2. Running the 4 Audit Scenes
To satisfy your project audit, you need to generate 4 distinct `.ppm` files. Run these commands in your terminal:

* **Scene 1 (Single Sphere):** A shiny red sphere.
  ```bash
  cargo run --release -- --scene 1 > scene1.ppm
  ```
* **Scene 2 (Plane & Low-Brightness Cube):** A matte cyan cube sitting on a floor, illuminated with a low-brightness light source.
  ```bash
  cargo run --release -- --scene 2 > scene2.ppm
  ```
* **Scene 3 (All Objects Showroom):** A beautiful setup containing a flat floor plane, a shiny blue sphere, a glossy green cube, and a shiny orange cylinder.
  ```bash
  cargo run --release -- --scene 3 > scene3.ppm
  ```
* **Scene 4 (Alternate Camera Angle):** The exact same setup as Scene 3, but viewed from a different perspective to demonstrate the rotatable camera system.
  ```bash
  cargo run --release -- --scene 4 > scene4.ppm
  ```

### 3. Rendering Previews (Fast Rendering)
High-resolution scenes (`800x600`) take a few seconds to render. During development or testing, you can render a preview **almost instantly** by reducing the dimensions:
```bash
cargo run -- --scene 3 --width 160 --height 120 > preview.ppm
```

### 4. Activating Bonus Flags (Textures & Reflections)
To keep rendering extremely fast by default, the bonus features are kept behind command-line flags (complying with the project guidelines):
* **Enable Textures (`-t` or `--textures`):** Activates checkerboard patterns on planes.
* **Enable Reflections (`-r` or `--reflections`):** Activates mirror reflections and glass refractions.

**Examples:**
```bash
# Render Scene 3 with checkerboard floor textures
cargo run --release -- --scene 3 -t > scene3_textured.ppm

# Render Scene 3 with textures AND shiny mirror/glass reflections
cargo run --release -- --scene 3 -t -r > scene3_premium.ppm
```

---

## Custom Scene File Format (`.rt`)

You can create a text file (e.g., `scene.rt`) to build custom scenes, translate shape positions, add lights, and move the camera directly from the terminal without editing any Rust source code.

### Syntax Rules
* Lines starting with `#` are comments.
* Fields are whitespace-separated.
* Coordinates are floats (`x y z`).
* Colors are float values (`r g b`) ranging from `0.0` (black) to `1.0` (white).

### Commands Reference

| Command | Syntax | Description |
| :--- | :--- | :--- |
| **camera** | `camera eye_x eye_y eye_z look_x look_y look_z fov` | Sets the camera position, target point, and Field of View (FOV). |
| **ambient** | `ambient r g b` | Sets the color and intensity of global ambient lighting. |
| **light** | `light x y z intensity r g b` | Creates a point light source with intensity and light color. |
| **sphere** | `sphere cx cy cz radius r g b specular reflective refractive transparency` | Adds a sphere with radius, color, specular highlight, reflectivity coefficient, index of refraction, and transparency coefficient. |
| **plane** | `plane px py pz nx ny nz r g b specular reflective [checker_freq c2_r c2_g c2_b]` | Adds an infinite flat plane with a point, a normal vector, color, specular, reflectivity. Optionally, append checker scale and a second color for procedural checkers. |
| **cube** | `cube min_x min_y min_z max_x max_y max_z r g b specular reflective` | Adds a cube defined by its minimum and maximum bounding corners. |
| **cylinder** | `cylinder cx cy cz radius height r g b specular reflective` | Adds a capped, Y-oriented cylinder. |

### Example `.rt` File
Save this as `my_scene.rt`:
```text
# 1. Camera positioned at (0, 1.5, 2) looking at (0, 0, -4) with 45 degrees FOV
camera 0 1.5 2 0 0 -4 45

# 2. Subtle white ambient light
ambient 0.1 0.1 0.1

# 3. Two light sources (one primary white, one weak secondary yellow)
light 5 8 -1 1.5 1.0 1.0 1.0
light -4 4 -2 0.4 1.0 0.9 0.5

# 4. Floor Plane (Checkerboard gray/white)
plane 0 -1 0 0 1 0 0.2 0.2 0.2 0.0 0.0 1.5 0.7 0.7 0.7

# 5. Mirror Sphere
sphere -1.5 0.0 -4.5 1.0 0.95 0.95 0.95 0.9 0.9 1.0 0.0

# 6. Glass Sphere
sphere 0.0 0.0 -3.5 0.8 1.0 1.0 1.0 0.9 0.1 1.5 0.9

# 7. Matte Orange Cylinder
cylinder 1.5 -1.0 -4.0 0.5 1.5 0.9 0.4 0.0 0.2 0.1
```

Render your custom scene file:
```bash
cargo run -- --file my_scene.rt -t -r > my_scene.ppm
```

---

## Code Examples (Internals)

### 1. Creating Shapes
All objects implement the `Intersect` trait. You can construct them with a designated `Material`:

```rust
// A shiny, 30% reflective blue sphere
let sphere_mat = Material::shiny(Vec3::new(0.1, 0.3, 0.9), 0.3);
let sphere = Sphere::new(
    Vec3::new(-1.6, -0.2, -4.2), // Position (Center)
    0.8,                         // Radius
    sphere_mat
);

// An infinite flat plane with a procedural checkerboard pattern
let plane_mat = Material::checker(
    Vec3::new(0.3, 0.3, 0.3),    // Primary gray
    Vec3::new(0.7, 0.7, 0.7),    // Secondary white
    1.5                          // Scale/Frequency
);
let plane = Plane::new(
    Vec3::new(0.0, -1.0, 0.0),   // Point on plane
    Vec3::new(0.0, 1.0, 0.0),    // Normal vector pointing UP
    plane_mat
);
```

### 2. Changing Brightness
The brightness of a scene can be controlled globally using the ambient light vector or on individual light sources:

```rust
// Define ambient light in the Scene struct
let ambient_light = Vec3::new(0.15, 0.15, 0.15); // Moderate ambient brightness

// Or configure direct point light intensities
let bright_light = Light::new(
    Vec3::new(4.0, 6.0, -1.0),
    1.5,                                         // High intensity (1.5)
    Vec3::new(1.0, 1.0, 1.0)
);

let dim_light = Light::new(
    Vec3::new(-3.0, 3.0, -2.0),
    0.4,                                         // Low intensity (0.4)
    Vec3::new(1.0, 1.0, 1.0)
);
```

### 3. Changing Camera Position and Angle
The `Camera` is fully configurable with view vectors computed relative to its look-at target:

```rust
let camera = Camera::new(
    Vec3::new(2.8, 2.0, -1.0),     // Camera Position (Eye)
    Vec3::new(0.0, -0.1, -4.0),    // Target Point (Look At)
    Vec3::new(0.0, 1.0, 0.0),      // Up Vector
    50.0,                          // Field of View in degrees
    aspect_ratio,                  // Viewport width / height
);
```