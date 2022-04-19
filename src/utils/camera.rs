

use crate::Vec3;
use crate::Ray;


// #[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}


impl Camera {

    pub fn new(
        look_from: Vec3, look_at: Vec3, view_up: Vec3,
        vfov: f64, aspect_ratio: f64,
        aperture: f64, focus_dist: f64,
    ) -> Camera {

        // fov
        let h = (vfov.to_radians() / 2.0).tan();

        // viewport
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // pan and tilt
        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        // camera
        let origin = look_from;
        let hor = focus_dist * viewport_width * u;
        let ver = focus_dist * viewport_height * v;
        let lower_left = origin - (hor / 2.0) - (ver / 2.0) - (focus_dist * w);

        Camera {
            u, v, w,
            lens_radius: aperture / 2.0,
            origin: origin,
            horizontal: hor,
            vertical: ver,
            lower_left: lower_left,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {

        let hor = self.horizontal * s;
        let ver = self.vertical * t;

        let rnd_radius = match self.lens_radius == 0.0 {
            true => Vec3::zero(),
            false => self.lens_radius * Vec3::random_in_unit_disc(),
        };
        let offset = (self.u * rnd_radius.x) + (self.v * rnd_radius.y);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + hor + ver - self.origin - offset,
        }
    }
}
