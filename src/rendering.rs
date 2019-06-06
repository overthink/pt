use crate::color::Color;
use crate::point::Point;
use crate::scene::{Element, Intersection, Plane, Scene, Sphere, SurfaceType};
use crate::vector::Vector3;

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

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

    pub fn create_reflection(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: f64,
    ) -> Ray {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - (2.0 * incident.dot(&normal) * normal),
        }
    }
}

#[derive(Debug)]
pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub trait Intersectable {
    // Returns distance from camera origin to point of intersection (if there is one)
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let vec_to_center: Vector3 = self.center - ray.origin;
        let adj: f64 = vec_to_center.dot(&ray.direction);
        let hyp2 = vec_to_center.dot(&vec_to_center); // len(v) == v.dot(v).sqrt()
        let opp2 = hyp2 - (adj * adj);
        let r2 = self.radius * self.radius;
        if opp2 > r2 {
            return None;
        }
        // There is an intersection, find distance from it to origin
        let thickness = (r2 - opp2).sqrt();

        // Ok, I don't get this part... :(
        // Something to do with a ray having two points of intersetcion with the sphere?
        let t0 = adj - thickness;
        let t1 = adj + thickness;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center; // vector from center to point of intersection
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / std::f32::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            // really close to zero == zero for us
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        // We need basis vectors for the plane. We'll get our x axis by crossing the surface normal
        // and the forward vector. If the surface normal happens to BE the forward vector, we'll
        // cross the normal with the up vector). This gives us a vector in our plane to be our x
        // axis. Then we cross that with the surface normal to get our y-axis.
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0, // forward vector
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0, // up vector
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);

        // Now we need to map the hit point to our new x and y axes. Do this by projecting the hit
        // vector onto each of our axes.
        let hit_vec = *hit_point - self.origin;

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, p: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref sphere) => sphere.surface_normal(p),
            Element::Plane(ref plane) => plane.surface_normal(p),
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
        }
    }
}

pub fn shade_diffuse(
    scene: &Scene,
    element: &Element,
    hit_point: Point,
    surface_normal: Vector3,
) -> Color {
    let texture_coords = element.texture_coords(&hit_point);

    let mut color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point); //.normalize();
        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };
        let in_light: bool = scene.trace(&shadow_ray).is_none();

        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        let light_power: f32 =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = element.material().albedo / std::f32::consts::PI;

        let light_color = light.color() * light_power * light_reflected;
        color = color + (element.material().coloration.color(&texture_coords) * light_color);
        // println!("{:?}", color);
    }

    color.clamp()
}

pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = shade_diffuse(scene, intersection.element, hit_point, surface_normal);
    if let SurfaceType::Reflective { reflectivity } = intersection.element.material().surface {
        let reflection_ray =
            Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
        color = color * (1.0 - reflectivity);
        color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    }
    color
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return BLACK;
    }

    let intersection = scene.trace(&ray);
    intersection
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(BLACK)
}
