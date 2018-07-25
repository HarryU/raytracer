use image::{Pixel, Rgba};
use point::Point;
use rendering::{Intersectable, Ray};
use std::ops::{Add, Mul};
use vector::Vector3;

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

#[derive(Deserialize, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

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

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
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

#[derive(Deserialize)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Deserialize)]
pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Deserialize)]
pub struct Plane {
    pub origin: Point,
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
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

#[derive(Deserialize)]
pub struct DirectionalLight {
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize)]
pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.color,
            Light::Spherical(ref s) => s.color,
        }
    }

    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_point).normalise(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).norm() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
}

#[derive(Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
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
