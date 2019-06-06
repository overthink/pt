extern crate pt;

use image::{DynamicImage, RgbaImage};
use pt::color::Color;
use pt::point::Point;
use pt::scene::{
    Coloration, DirectionalLight, Element, Light, Material, Plane, Scene, Sphere, SphericalLight,
    SurfaceType,
};
use pt::vector::Vector3;

// ideally we could load the texture once, but somewhere downstream it gets consumed -- I think the
// get_pixel call in the renderer?
fn checkerboard() -> DynamicImage {
    image::open("src\\bin\\checkerboard.png").unwrap()
}

fn scene() -> Scene {
    Scene {
        width: 1600,
        height: 900,
        fov: 90.0,
        elements: vec![
            Element::Sphere(Sphere {
                center: Point {
                    x: 0.3,
                    y: 0.5,
                    z: -3.0,
                },
                radius: 0.85,
                material: Material {
                    coloration: Coloration::Color(Color {
                        red: 0.0,
                        green: 1.0,
                        blue: 0.0,
                    }),
                    albedo: 5.0,
                    surface: SurfaceType::Reflective { reflectivity: 0.3 },
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: 3.5,
                    y: -0.2,
                    z: -6.0,
                },
                radius: 0.5,
                material: Material {
                    coloration: Coloration::Color(Color {
                        red: 1.0,
                        green: 0.0,
                        blue: 0.0,
                    }),
                    albedo: 3.0,
                    surface: SurfaceType::Reflective {
                        reflectivity: 0.001,
                    },
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: -2.5,
                    y: 2.0,
                    z: -6.0,
                },
                radius: 2.0,
                material: Material {
                    coloration: Coloration::Texture(checkerboard()),
                    albedo: 6.0,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Element::Plane(Plane {
                origin: Point {
                    x: 0.0,
                    y: -2.0,
                    z: 0.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                material: Material {
                    coloration: Coloration::Texture(checkerboard()),
                    albedo: 1.0,
                    surface: SurfaceType::Reflective { reflectivity: 0.5 },
                },
            }),
        ],
        shadow_bias: 1e-13,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: -0.8,
                    y: -1.0,
                    z: -0.4,
                },
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 0.8,
            }),
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: 0.8,
                    y: -1.0,
                    z: -0.4,
                },
                color: Color {
                    red: 1.0,
                    green: 0.2,
                    blue: 0.2,
                },
                intensity: 0.6,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: -1.0,
                    y: 1.5,
                    z: -1.0,
                },
                color: Color {
                    red: 0.1,
                    green: 0.1,
                    blue: 0.8,
                },
                intensity: 100.0,
            }),
        ],
        max_recursion_depth: 3,
    }
}

// Entry point for creating renderings.
fn main() {
    let scene = scene();
    let img: RgbaImage = pt::render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    img.save("c:\\temp\\foo.png").unwrap();
}
