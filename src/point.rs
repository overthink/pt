use crate::vector::Vector3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
