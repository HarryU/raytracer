extern crate image;

mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage};
use point::Point;
use rendering::{Intersectable, Ray};
use scene::{Color, Element, Intersection, Light, Plane, Scene, Sphere};
use vector::Vector3;

fn main() {
    let test_scene = Scene {
        width: 1920,
        height: 1080,
        fov: 90.0,
        shadow_bias: 1e-12,
        elements: vec![
            Element::Sphere(Sphere {
                centre: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.2,
                    green: 1.0,
                    blue: 0.2,
                },
                albedo: 0.18,
            }),
            Element::Sphere(Sphere {
                centre: Point {
                    x: -3.0,
                    y: 1.0,
                    z: -6.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.8,
                    green: 0.2,
                    blue: 0.4,
                },
                albedo: 0.58,
            }),
            Element::Sphere(Sphere {
                centre: Point {
                    x: 2.0,
                    y: 1.0,
                    z: -4.0,
                },
                radius: 1.5,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                albedo: 0.18,
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
                color: Color {
                    red: 0.5,
                    blue: 0.5,
                    green: 0.5,
                },
                albedo: 0.18,
            }),
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
                color: Color {
                    red: 0.2,
                    blue: 0.3,
                    green: 1.0,
                },
                albedo: 0.38,
            }),
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
            intensity: 15.0,
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
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let direction_to_light = -scene.light.direction.normalise();
    let shadow_ray = Ray {
        origin: hit_point + (direction_to_light * scene.shadow_bias),
        direction: direction_to_light,
    };
    let in_light = scene.trace(&shadow_ray).is_none();
    let light_intensity = if in_light { scene.light.intensity } else { 0.0 };
    let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
    let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
    let color = intersection.element.color() * scene.light.color * light_power * light_reflected;
    color.clamp()
}
