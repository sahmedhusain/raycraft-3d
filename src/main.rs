mod camera;
mod light;
mod material;
mod object;
mod ppm;
mod ray;
mod renderer;
mod scene;
mod vec3;

use scene::Scene;
use std::env;
use std::process;

fn print_help() {
    eprintln!(
        "rt - 3D Ray Tracer

Usage:
  cargo run [options] > output.ppm
  cargo run -- [options] > output.ppm

Options:
  --scene <1-4>   Select a built-in audit scene (default: 3)
                  1: Single Sphere Scene
                  2: Plane & Low-Brightness Cube Scene
                  3: Complete Showroom (All 4 shapes)
                  4: Showroom from an alternate camera perspective
  --file <path>   Load a custom scene configuration from a .rt file
  --width <px>    Override viewport width in pixels (default: 800)
  --height <px>   Override viewport height in pixels (default: 600)
  -t, --textures  Enable surface textures (e.g., checkerboards)
  -r, --reflections Enable mirror reflections and transparent glass refractions
  -p, --particles Enable rendering of sparkles/dust particles
  -f, --fluids    Enable transparent refractive fluid plane
  -h, --help      Display this help menu
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // defaults
    let mut scene_num = 3;
    let mut file_path: Option<String> = None;
    let mut width = 800;
    let mut height = 600;
    let mut enable_textures = false;
    let mut enable_reflections = false;
    let mut enable_particles = false;
    let mut enable_fluids = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--scene" => {
                if i + 1 < args.len() {
                    scene_num = args[i + 1].parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid scene index. Must be 1, 2, 3 or 4.");
                        process::exit(1);
                    });
                    if scene_num < 1 || scene_num > 4 {
                        eprintln!("Error: Scene index must be between 1 and 4.");
                        process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --scene parameter.");
                    process::exit(1);
                }
            }
            "--file" => {
                if i + 1 < args.len() {
                    file_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Missing path for --file parameter.");
                    process::exit(1);
                }
            }
            "--width" => {
                if i + 1 < args.len() {
                    width = args[i + 1].parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid width value.");
                        process::exit(1);
                    });
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --width parameter.");
                    process::exit(1);
                }
            }
            "--height" => {
                if i + 1 < args.len() {
                    height = args[i + 1].parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid height value.");
                        process::exit(1);
                    });
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --height parameter.");
                    process::exit(1);
                }
            }
            "-t" | "--textures" => {
                enable_textures = true;
                i += 1;
            }
            "-r" | "--reflections" => {
                enable_reflections = true;
                i += 1;
            }
            "-p" | "--particles" => {
                enable_particles = true;
                i += 1;
            }
            "-f" | "--fluids" => {
                enable_fluids = true;
                i += 1;
            }
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            _ => {
                eprintln!(
                    "Error: Unknown option '{}'. Use -h or --help for instructions.",
                    args[i]
                );
                process::exit(1);
            }
        }
    }

    let aspect_ratio = width as f64 / height as f64;

    let scene = if let Some(path) = file_path {
        eprintln!("Info: Loading scene file '{}'...", path);
        Scene::load_from_file(&path, aspect_ratio).unwrap_or_else(|err| {
            eprintln!("Scene Parse Error: {}", err);
            process::exit(1);
        })
    } else {
        eprintln!("Info: Loading built-in Scene {}...", scene_num);
        match scene_num {
            1 => Scene::build_scene_1(aspect_ratio),
            2 => Scene::build_scene_2(aspect_ratio),
            3 => Scene::build_scene_3(aspect_ratio),
            4 => Scene::build_scene_4(aspect_ratio),
            _ => unreachable!(),
        }
    };

    eprintln!(
        "Info: Commencing render [Resolution: {}x{}]...",
        width, height
    );

    let start_time = std::time::Instant::now();

    let config = renderer::RenderConfig {
        enable_textures,
        enable_reflections,
        enable_particles,
        enable_fluids,
    };

    let pixels = renderer::render(&scene, width, height, config);

    let duration = start_time.elapsed();
    eprintln!("Info: Render completed in {:.2?}!", duration);

    eprintln!("Info: Generating PPM image data...");
    let ppm_data = ppm::write_ppm(&pixels, width, height);

    print!("{}", ppm_data);
    eprintln!("Info: PPM output printed to stdout successfully!");
}
