#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod matrix;
mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage};
use point::Point;
use rendering::{cast_ray, Ray};
use scene::{
    Camera, Color, Coloration, Element, Light, Material, Plane, Scene, Sphere,
    SphericalLight, SurfaceType,
};
use std::fs::File;
use vector::Vector3;
use std::path::Path;
use std::ffi::OsStr;
use clap::{Arg, App, SubCommand};
use rand::Rng;


fn main() {
    let matches = App::new("Rust Raytracer")
        .version("1.0")
        .author("Harry Uzzell <hmucs@yahoo.co.uk>")
        .about("A minimal raytracer implmented in Rust")
        .arg(Arg::with_name("input_file")
            .short("i")
            .long("input_file")
            .value_name("FILE")
            .help("Sets an input scene file")
            .takes_value(true))
        .subcommand(SubCommand::with_name("random")
            .about("Specify a grid to populate with random shapes")          
            .arg(Arg::with_name("x")
                .help("the size of the random shape grid in x")
                .index(1)
                .required(true))
            .arg(Arg::with_name("y")
                .help("the size of the random shape grid in y")
                .index(2)
                .required(true)))
        .get_matches();
    let mut scene: Scene = if matches.is_present("input_file") {
        let filename = matches.value_of("input_file").unwrap();
        let scene_file = File::open(filename).expect("File not found");
        let extension = Path::new(filename).extension().and_then(OsStr::to_str);
        if extension == Some("json") {
            serde_json::from_reader(scene_file).unwrap()
        } else if extension == Some("yml") {
            serde_yaml::from_reader(scene_file).unwrap()
        } else {
            panic!("An input file path was provided but it wasn't a json or yml file.")
        }
    } else if let Some(matches) = matches.subcommand_matches("random") {
        let x = value_t!(matches, "x", i32).unwrap_or(3);
        let y = value_t!(matches, "y", i32).unwrap_or(3);
        random_shapes(x, y)
    } else {
        random_shapes(3, 3)
    };
    scene.camera.rotation_matrix = Camera::calculate_rotation_matrix(
        scene.camera.look_at,
        scene.camera.position,
        scene.camera.up,
    );
    let img: DynamicImage = render(&scene);
    img.save("output.png").expect("Failed to save output image");
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let mut color = Color::black();
            for _ in 0..scene.n_samples {
                let ray = Ray::create_prime(
                    (x as f32) + (rand::random::<f32>() - 0.5),
                    (y as f32) + (rand::random::<f32>() - 0.5),
                    scene,
                );
                color = color + cast_ray(scene, &ray, 0);
            }
            color = color * (1.0 / scene.n_samples as f32);
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

fn random_shapes(rows: i32, cols: i32) -> Scene {
    let mut elements: Vec<Element> = vec![
        Element::Plane(Plane {..Default::default()}),
        Element::Plane(Plane {
            origin: Point {
                x: 0.0,
                y: -2.0,
                z: -5.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            ..Default::default()}),
    ];
    for a in 0..rows {
        for b in 0..cols {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0.4, 0.8);
            let x = ((a - 1) * 3) as f32 + rng.gen_range(0.0, 0.9);
            let y = -2.0 + r;
            let z = ((b - 4) * 3) as f32 + rng.gen_range(0.0, 0.9);
            let shape = Element::Sphere(
                Sphere{
                    centre: Point {
                        x: (x).into(),
                        y: y,
                        z: (z).into(),
                    },
                    radius: r,
                    ..Default::default()});
            elements.push(shape);
        }
    }

    let position = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let look_at = Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let camera = Camera {
        position: position,
        look_at: look_at,
        up: up,
        rotation_matrix: Camera::calculate_rotation_matrix(look_at, position, up),
    };

    let lights: Vec<Light> = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: 5.0,
                y: 10.0,
                z: 0.0,
            },
            color: Color::white(),
            intensity: 15000.0,
        }),
        Light::Spherical(SphericalLight {
            position: Point {
                x: -5.0,
                y: 10.0,
                z: 0.0,
            },
            color: Color::white(),
            intensity: 15000.0,
        }),
    ];

    Scene {
        width: 800,
        height: 400,
        elements: elements,
        lights: lights,
        camera: camera,
        fov: 90.0,
        shadow_bias: 1e-10,
        max_recursion_depth: 6,
        n_samples: 90,
    }
}
