use image;
use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use matrix::Matrix33;
use point::Point;
use rendering::{Intersectable, Ray, TextureCoords};
use serde;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::ops::{Add, Mul};
use std::path::PathBuf;
use vector::Vector3;

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Coloration {
    Color(Color),
    Texture(#[serde(deserialize_with = "load_texture", serialize_with = "write_string")] DynamicImage),
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(c) => c,
            Coloration::Texture(ref tex) => {
                let tex_x = wrap(texture_coords.x, tex.width());
                let tex_y = wrap(texture_coords.y, tex.height());
                Color::from_rgba(tex.get_pixel(tex_x, tex_y))
            }
        }
    }
}

pub fn load_texture<'de, D>(deserializer: D) -> Result<DynamicImage, D::Error>
where
    D: Deserializer<'de>,
{
    let path = PathBuf::deserialize(deserializer)?;
    Ok(image::open(path).expect("Unable to open texture file"))
}

pub fn write_string<S>(_: &DynamicImage, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("texture file")
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba[0] as f32) / 255.0),
            green: gamma_decode((rgba[1] as f32) / 255.0),
            blue: gamma_decode((rgba[2] as f32) / 255.0),
        }
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

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        self * other as f32
    }
}

#[derive(Serialize, Deserialize)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
    Disk(Disk),
}

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub origin: Point,
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub normal: Vector3,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub struct Disk {
    pub origin: Point,
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub normal: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Element {
    pub fn color(&self, hit: &Point) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.coloration.color(&self.texture_coords(hit)),
            Element::Plane(ref p) => p.material.coloration.color(&self.texture_coords(hit)),
            Element::Disk(ref d) => d.material.coloration.color(&self.texture_coords(hit)),
        }
    }

    pub fn albedo(&self) -> &f32 {
        match *self {
            Element::Sphere(ref s) => &s.material.albedo,
            Element::Plane(ref p) => &p.material.albedo,
            Element::Disk(ref d) => &d.material.albedo,
        }
    }

    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
            Element::Disk(ref d) => &d.material,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Point,
    #[serde(default = "Point::default_look_at")]
    pub look_at: Point,
    #[serde(default = "Vector3::default_up")]
    pub up: Vector3,
    #[serde(skip_deserializing, skip_serializing)]
    pub rotation_matrix: Matrix33,
}

impl Camera {
    pub fn calculate_rotation_matrix(
        look_at: Point,
        position: Point,
        world_up: Vector3,
    ) -> Matrix33 {
        let forward = (position - look_at).normalise();
        let right = world_up.cross(&forward).normalise();
        let up = forward.cross(&right).normalise();
        Matrix33::from_vecs(&right, &up, &forward)
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub n_samples: u32,
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
