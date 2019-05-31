// https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

extern crate image;

pub mod color;
pub mod point;
mod rendering;
pub mod scene;
pub mod vector;

use image::Rgba;
use image::RgbaImage;

use crate::rendering::{get_color, Ray};
use crate::scene::Scene;

pub fn render(scene: &Scene) -> RgbaImage {
    let mut image = RgbaImage::new(scene.width, scene.height);
    let sky = Rgba([178, 212, 255, 255]);

    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = match intersection {
                Some(intersection) => get_color(scene, &ray, &intersection).to_rgba(),
                _ => sky,
            };
            image.put_pixel(x, y, color);
        }
    }
    image
}
