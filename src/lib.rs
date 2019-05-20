// https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

extern crate image;
extern crate rand;

pub mod color;
pub mod point;
mod rendering;
pub mod scene;
pub mod vector;

use image::RgbImage;

use crate::color::Color;
use crate::point::Point;
use crate::scene::{Scene, Sphere};

pub fn render(scene: &Scene) -> RgbImage {
    //DynamicImage::new_rgb8(scene.width, scene.height)
    RgbImage::new(scene.width, scene.height)
}

#[test]
fn test_can_render_scene() {
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
}
