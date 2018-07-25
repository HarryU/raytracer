#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;
extern crate serde_json;
use std::fs::File;

mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage};
use rendering::{Intersectable, Ray};
use scene::{Color, Intersection, Scene};

fn main() {
    let scene_file = File::open("test_scene.json").expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();
    let img: DynamicImage = render(&scene);
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
