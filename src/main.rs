extern crate image;

mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage};
use point::Point;
use rendering::{Intersectable, Ray};
use scene::{
    Color, DirectionalLight, Element, Intersection, Light, Plane, Scene, Sphere, SphericalLight,
};
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
        lights: vec![
            Light::Directional(DirectionalLight {
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
                intensity: 0.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: -2.0,
                    y: 10.0,
                    z: -3.0,
                },
                color: Color {
                    red: 0.3,
                    green: 0.8,
                    blue: 0.3,
                },
                intensity: 40000.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: 0.25,
                    y: 0.0,
                    z: -2.0,
                },
                color: Color {
                    red: 0.8,
                    green: 0.3,
                    blue: 0.3,
                },
                intensity: 1000.0,
            }),
        ],
    };

    let img: DynamicImage = render(&test_scene);
    img.save("output.png").expect("Failed to save output image");
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = intersection
                .map(|i| get_color(&scene, &ray, &i))
                .unwrap_or(Color::black());
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin.clone() + (ray.direction.clone() * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let mut color = Color::black();
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point);
        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);
        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
        let light_color = light.color() * light_power * light_reflected;
        color = color + (intersection.element.color() * light_color);
    }
    color.clamp()
}
