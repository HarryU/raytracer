use point::Point;
use scene::{Color, Disk, Element, Intersection, Plane, Scene, Sphere, SurfaceType};
use std::f32;
use std::f32::consts::PI;
use vector::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        let w = scene.camera.position - scene.camera.look_direction;
        let u = scene.camera.up.cross(&w.to_vector()).normalise();
        let v = w.to_vector().cross(&u);
        let direction = Vector3 {
            x: sensor_x,
            y: sensor_y,
            z: -1.,
        };

        Ray {
            origin: scene.camera.position,
            direction: direction.normalise(),
        }
    }

    pub fn create_reflection(
        surface_normal: Vector3,
        incident_direction: Vector3,
        surface_intersection: Point,
        shadow_bias: f64,
    ) -> Ray {
        Ray {
            origin: surface_intersection + (surface_normal * shadow_bias),
            direction: incident_direction
                - (2.0 * incident_direction.dot(&surface_normal) * surface_normal),
        }
    }

    pub fn create_transmission(
        normal: Vector3,
        incident: Vector3,
        surface_intersection: Point,
        shadow_bias: f64,
        index: f32,
    ) -> Option<Ray> {
        let mut ref_normal = normal;
        let mut eta_t = index as f64;
        let mut eta_i = 1.0f64;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            i_dot_n = -i_dot_n;
        } else {
            ref_normal = -normal;
            eta_t = 1.0;
            eta_i = index as f64;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: surface_intersection + (ref_normal * -shadow_bias),
                direction: (incident + ref_normal * i_dot_n) * eta - ref_normal * k.sqrt(),
            })
        }
    }
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
            Element::Disk(ref d) => d.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
            Element::Disk(ref d) => d.surface_normal(hit_point),
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
            Element::Disk(ref d) => d.texture_coords(hit_point),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let line: Vector3 = self.centre - ray.origin;

        let adj = line.dot(&ray.direction);
        let d2 = line.dot(&line) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > (radius2) {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        let surface_normal = (hit_point.clone() - self.centre.clone()).normalise();
        surface_normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.centre;
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / f32::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / f32::consts::PI,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.origin;

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

impl Intersectable for Disk {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let plane = Plane {
            origin: self.origin,
            normal: self.normal,
            material: self.material.clone(),
        };
        let plane_intersection = plane.intersect(ray);
        if plane_intersection.is_some() {
            let v = (ray.origin + (ray.direction * plane_intersection.unwrap())) - self.origin;
            if v.dot(&v) < (self.radius * self.radius) {
                plane_intersection
            } else {
                None
            }
        } else {
            None
        }
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.origin;

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return Color::black();
    }

    let intersection = scene.trace(&ray);
    intersection
        .map(|i| get_color(&scene, &ray, &i, depth))
        .unwrap_or(Color::black())
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let material = intersection.element.material();
    match material.surface {
        SurfaceType::Diffuse => {
            diffuse_color(scene, intersection.element, hit_point, surface_normal)
        }
        SurfaceType::Reflective { reflectivity } => {
            let mut color = diffuse_color(scene, intersection.element, hit_point, surface_normal);
            let reflection_ray =
                Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            color = color * (1.0 - reflectivity);
            color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity)
        }
        SurfaceType::Refractive {
            index,
            transparency,
        } => {
            let mut refraction_color = Color::black();
            let kr = fresnel(ray.direction, surface_normal, index);
            let surface_color = material
                .coloration
                .color(&intersection.element.texture_coords(&hit_point));

            if kr < 1.0 {
                let transmission_ray = Ray::create_transmission(
                    surface_normal,
                    ray.direction,
                    hit_point,
                    scene.shadow_bias,
                    index,
                ).unwrap();
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray =
                Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            color = color * transparency * surface_color;
            color
        }
    }
}

fn diffuse_color(
    scene: &Scene,
    element: &Element,
    hit_point: Point,
    surface_normal: Vector3,
) -> Color {
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
        let light_reflected = element.albedo() / PI;
        let light_color = light.color() * light_power * light_reflected;
        color = color + (element.color(&hit_point) * light_color);
    }
    color.clamp()
}

fn fresnel(incident: Vector3, normal: Vector3, index: f32) -> f64 {
    let mut eta_t = index as f64;
    let mut eta_i = 1.0f64;
    let mut i_dot_n = incident.dot(&normal);
    if i_dot_n > 0.0 {
        eta_t = 1.0;
        eta_i = index as f64;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        return 1.0;
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_t) - (eta_t * cos_i)) / ((eta_i * cos_t) + (eta_t * cos_i));
        return (r_s * r_s + r_p * r_p) / 2.0;
    }
}
