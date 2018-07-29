#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod matrix;
mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage};
use point::Point;
use rendering::{cast_ray, Intersectable, Ray};
use scene::{Camera, Color, Intersection, Scene};
use scene::{Coloration, Element, Light, Material, Plane, Sphere, SphericalLight, SurfaceType};
use std::fs::File;
use vector::Vector3;

fn main() {
    //let scene_file = File::open("test_scene.json").expect("File not found");
    //let scene: Scene = serde_json::from_reader(scene_file).unwrap();
    let scene = random_scene();
    let img: DynamicImage = render(&scene);
    img.save("output.png").expect("Failed to save output image");
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
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
                albedo: 0.5,
                surface: SurfaceType::Reflective { reflectivity: 0.1 },
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
                    red: 0.5,
                    green: 0.0,
                    blue: 0.6,
                }),
                albedo: 0.25,
                surface: SurfaceType::Diffuse,
            },
        }),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let sphere = Element::Sphere(Sphere {
                centre: Point {
                    x: (a as f32 + 0.9 * rand::random::<f32>()).into(),
                    y: -1.8,
                    z: (b as f32 + 0.9 * rand::random::<f32>()).into(),
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

    let camera = Camera {
        position: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        look_at: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        up: Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };

    let lights: Vec<Light> = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: 5.0,
                y: 25.0,
                z: 15.0,
            },
            color: Color::white(),
            intensity: 10000.0,
        }),
        Light::Spherical(SphericalLight {
            position: Point {
                x: -5.0,
                y: 5.0,
                z: -5.0,
            },
            color: Color {
                red: 0.8,
                green: 0.8,
                blue: 0.2,
            },
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
    }
}
