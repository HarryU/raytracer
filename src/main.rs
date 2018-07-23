extern crate image;

mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage, Pixel, Rgba};
use point::Point;
use rendering::{Intersectable, Ray};
use scene::{Color, Intersection, Light, Scene, Sphere};
use vector::Vector3;

fn main() {
    let test_scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![
            Sphere {
                centre: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.5,
                color: Color {
                    red: 0.4,
                    green: 0.7,
                    blue: 0.3,
                },
                albedo: 0.22,
            },
            Sphere {
                centre: Point {
                    x: -3.0,
                    y: 2.0,
                    z: -7.0,
                },
                radius: 3.0,
                color: Color {
                    red: 0.6,
                    green: 0.1,
                    blue: 0.5,
                },
                albedo: 0.18,
            },
        ],
        light: Light {
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 10.0,
        },
    };

    let img: DynamicImage = render(&test_scene);
    img.save("output.png").expect("Failed to save output image");
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = intersection
                .map(|i| get_color(&scene, &ray, &i))
                .unwrap_or(black);
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin.clone() + (ray.direction.clone() * intersection.distance);
    let surface_normal = intersection.sphere.surface_normal(&hit_point);
    let direction_to_light = -scene.light.direction.normalise();
    let light_power =
        (surface_normal.dot(&direction_to_light) as f32).max(0.0) * scene.light.intensity;
    let light_reflected = intersection.sphere.albedo / std::f32::consts::PI;
    let color = intersection.sphere.color.clone()
        * scene.light.color.clone()
        * light_power
        * light_reflected;
    color.clamp()
}
