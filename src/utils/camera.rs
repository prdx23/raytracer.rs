

use crate::Vec3;
use crate::Ray;


// #[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}


impl Camera {

    pub fn new() -> Camera {

        let aspect_ratio = 16.0 / 9.0;

        // viewport
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let focal_vector = Vec3::new(0.0, 0.0, focal_length);

        // camera
        let origin = Vec3::zero();
        let hor = Vec3::new(viewport_width, 0.0, 0.0);
        let ver = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left = origin - (hor / 2.0) - (ver / 2.0) - focal_vector;

        Camera {
            origin: origin,
            horizontal: hor,
            vertical: ver,
            lower_left: lower_left,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let hor = self.horizontal * u;
        let ver = self.vertical * v;
        Ray { origin: self.origin, direction: self.lower_left + hor + ver }
    }
}
