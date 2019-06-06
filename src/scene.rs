use crate::color::Color;
use crate::point::Point;
use crate::rendering::{Intersectable, Ray, TextureCoords};
use crate::vector::Vector3;
use image::{DynamicImage, GenericImageView};
use std::fmt::{Error, Formatter};

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage),
}

impl std::fmt::Debug for Coloration {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Coloration::Color(ref c) => c.fmt(f),
            Coloration::Texture(_) => write!(f, "RgbaImage"),
        }
    }
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

impl Coloration {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(c) => c.clone(),
            Coloration::Texture(ref tex) => {
                let tex_x = wrap(coords.x, tex.width());
                let tex_y = wrap(coords.y, tex.height());
                Color::from_rgba(tex.get_pixel(tex_x, tex_y))
            }
        }
    }
}

#[derive(Debug)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
}

#[derive(Debug)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType,
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}

#[derive(Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub material: Material,
}

#[derive(Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn material(&self) -> &Material {
        match self {
            Element::Sphere(s) => &s.material,
            Element::Plane(p) => &p.material,
        }
    }
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, element: &Element) -> Intersection {
        if !distance.is_finite() {
            panic!("Intersection must have finite distance");
        }
        Intersection { distance, element }
    }
}

// Light so far away that all rays coming from it are effectively parallel
#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

// Point source light near the scene
#[derive(Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug)]
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

    // Returns a vector pointing to the light from the given point.
    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let p = s.position - *hit_point;
                let r2 = (p.x * p.x + p.y * p.y + p.z * p.z) as f32;
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

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64, // hack to ensure intersection points are outside their elements
    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
