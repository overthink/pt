extern crate pt;

use pt::color::Color;
use pt::point::Point;
use pt::scene::{Element, Plane, Scene, Sphere};
use pt::vector::Vector3;

fn scene() -> Scene {
    Scene {
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
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
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
                    green: 0.0,
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
                    green: 0.0,
                    blue: 1.0,
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
                color: Color {
                    red: 0.3,
                    green: 0.3,
                    blue: 0.3,
                },
            }),
        ],
    }
}

// Entry point for creating renderings.
fn main() {
    let scene = scene();
    let img = pt::render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    img.save("c:\\temp\\foo.png").unwrap();
}
