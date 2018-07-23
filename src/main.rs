extern crate image;

mod point;
mod rendering;
mod scene;
mod vector;

use image::{DynamicImage, GenericImage, Pixel, Rgba};
use point::Point;
use rendering::Ray;
use scene::{Color, Scene, Sphere};

fn main() {
    let test_scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![Sphere {
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
        }],
    };

    let img: DynamicImage = render(&test_scene);
    img.save("output.png").expect("Failed to save output image");
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = intersection
                .map(|i| Color::to_rgba(&i.sphere.color))
                .unwrap_or(black);
            image.put_pixel(x, y, color);
        }
    }
    image
}
