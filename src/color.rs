use image::Rgba;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        ])
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}
