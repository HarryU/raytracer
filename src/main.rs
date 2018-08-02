#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate image;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod matrix;
mod point;
mod rendering;
mod scene;
mod vector;

use clap::{App, Arg};
use image::{DynamicImage, GenericImage};
use point::Point;
use rendering::{cast_ray, Intersectable, Ray};
use scene::{
    Camera, Color, Coloration, Element, Intersection, Light, Material, Plane, Scene, Sphere,
    SphericalLight, SurfaceType,
};
use std::env;
use std::fs::File;
use vector::Vector3;

fn main() {
    //let scene_file = File::open("test_scene.json").expect("File not found");
    //let mut scene: Scene = serde_json::from_reader(scene_file).unwrap();
    //scene.camera.rotation_matrix = Camera::calculate_rotation_matrix(
    //    scene.camera.look_at,
    //    scene.camera.position,
    //    scene.camera.up,
    //);
    let scene = random_scene();
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

fn random_scene() -> Scene {
    let mut elements: Vec<Element> = vec![
        Element::Plane(Plane {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -20.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 0.4,
                    green: 0.5,
                    blue: 0.65,
                }),
                albedo: 0.23,
                surface: SurfaceType::Reflective { reflectivity: 0.3 },
            },
        }),
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
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 0.7,
                    green: 0.4,
                    blue: 0.8,
                }),
                albedo: 0.68,
                surface: SurfaceType::Diffuse,
            },
        }),
    ];
    for a in -1..1 {
        for b in -2..-1 {
            let sphere = Element::Sphere(Sphere {
                centre: Point {
                    x: ((a * 2) as f32 + 0.9 * rand::random::<f32>()).into(),
                    y: -1.8,
                    z: ((b * 2) as f32 + 0.9 * rand::random::<f32>()).into(),
                },
                radius: 0.2,
                material: Material {
                    coloration: Coloration::Color(Color {
                        red: rand::random::<f32>(),
                        blue: rand::random::<f32>(),
                        green: rand::random::<f32>(),
                    }),
                    albedo: rand::random::<f32>(),
                    surface: SurfaceType::Diffuse,
                },
            });
            elements.push(sphere);
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
        width: 1920,
        height: 1080,
        elements: elements,
        lights: lights,
        camera: camera,
        fov: 90.0,
        shadow_bias: 1e-10,
        max_recursion_depth: 6,
        n_samples: 90,
    }
}
