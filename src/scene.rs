use image::{Pixel, Rgba};
use point::Point;
use rendering::{Intersectable, Ray};
use std::ops::Mul;
use vector::Vector3;

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

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.color,
            Element::Plane(ref p) => p.color,
        }
    }

    pub fn albedo(&self) -> &f32 {
        match *self {
            Element::Sphere(ref s) => &s.albedo,
            Element::Plane(ref p) => &p.albedo,
        }
    }
}

pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
    pub elements: Vec<Element>,
    pub light: Light,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
    //Secret variable stops outside code constructing this; have to use new instead.
    _secret: (),
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have finite distance.");
        }
        Intersection {
            distance: distance,
            element: element,
            _secret: (),
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
