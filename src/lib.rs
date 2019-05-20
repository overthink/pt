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

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba());
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
    use crate::scene::Sphere;
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
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
        },
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    img.save("c:\\temp\\foo.png").unwrap();
}
