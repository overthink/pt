use crate::point::Point;
use crate::scene::Scene;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // Recall: pos x is right, pos y is up, pos z is coming out of screen towards us
        // Camera is at (0, 0, 0)
        // Assume we have a 2x2 unit camera sensor/film plane one unit in front of the camera
        // Coordinates of the sensor will be -1.0..1.0 x -1.0..1.0 (like in OpenGL).
        // screen pixels: 0,0 is in the top left
        assert!(scene.width > scene.height); // limitation for now

        // fov: our working model is that the sensor is 1.0 units in front of the camera. If fov is
        // 90 degrees everything happens to work out. But if fov is, say, 120 degrees we have a
        // problem if the camera is still 1.0 units away from the sensor: some rays in the fov will
        // miss the sensor. With some trig we can adjust the sensor size (keeping it 1.0 units from
        // the camera) to account for this.
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();

        // aspect ratio: If we have a square sensor on on camera (as we do: -1.0..1.0 x -1.0..1.0)
        // but a non-square screen, we will have non-square pixels on the sensor, which will cause a
        // distortion. Multiplying x by aspect ratio will fix this, but also... enlarge? the sensor.
        // (i.e. 16 aspect-ratio corrected pixels on the sensor have width > 2.0). I think changing
        // the sensor size like this works ok becuase the sensor coordinates have (0, 0) at the
        // centre, so it grows equally in all directions. TBD if I really get this, but it is easy
        // to see what is happening if you draw out dividing a 2.0x2.0 unit sensor into 16x9 pixels.
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);

        // Map a screen coordinate to sensor space. The +0.5 stuff is because we want rays to pass
        // through the center of a pixel on the sesor, not the top-left corner. The `1.0-` for the y
        // coord is because screen pixels have positive y pointing down, but sensor coords have
        // positive y pointing up.
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}
