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
use rendering::{cast_ray, Intersectable, Ray};
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
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
    image
}
