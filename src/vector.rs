use std::ops::Mul;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // aka manitude, Eudclidian norm, l2-norm
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y * self.z * self.z).sqrt()
    }

    // divide each vector component by its length
    pub fn normalize(&self) -> Vector3 {
        self * self.length().recip()
    }
}
