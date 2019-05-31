use crate::color::Color;
use crate::point::Point;
use crate::rendering::{Intersectable, Ray};
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

impl Sphere {
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
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

#[derive(Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => p.albedo,
        }
    }
}

#[derive(Debug)]
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Light,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
