use image::{Pixel, Rgba};
use point::Point;
use rendering::{Intersectable, Ray};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spheres: Vec<Sphere>,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub sphere: &'a Sphere,
    //Secret variable stops outside code constructing this; have to use new instead.
    _secret: (),
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, sphere: &'b Sphere) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have finite distance.");
        }
        Intersection {
            distance: distance,
            sphere: sphere,
            _secret: (),
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
