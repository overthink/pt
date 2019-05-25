// https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

extern crate image;
extern crate rand;

pub mod color;
pub mod point;
mod rendering;
pub mod scene;
pub mod vector;

use image::Rgba;
use image::RgbaImage;

use crate::rendering::{Intersectable, Ray};
use crate::scene::Scene;

pub fn render(scene: &Scene) -> RgbaImage {
    let mut image = RgbaImage::new(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 255]);

    let hack = Rgba([255, 255, 0, 255]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            if intersection.is_some() {
                image.put_pixel(x, y, hack);
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

#[test]
fn test_can_render_scene() {
    use crate::color::Color;
    use crate::point::Point;
    use crate::scene::{Element, Plane, Sphere};
    use crate::vector::Vector3;
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec![
            Element::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: 3.5,
                    y: 0.0,
                    z: -6.0,
                },
                radius: 0.5,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: -4.5,
                    y: 2.0,
                    z: -7.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            }),
            Element::Plane(Plane {
                origin: Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                },
            }),
        ],
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    img.save("c:\\temp\\foo.png").unwrap();
}
