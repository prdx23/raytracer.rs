
pub mod sphere;
pub mod cube;
pub mod world;

use crate::Vec3;
use crate::Ray;


pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitDetail>;
}

// use std::fmt;
// impl fmt::Debug for dyn Hit {
//     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // write!(fmt, "{:?}", self.center)
//         write!(fmt, "<object>")
//     }
// }

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct HitDetail {
    t: f64,
    point: Vec3,
    normal: Vec3,
    front_face: bool,
    calc: bool,
}

impl HitDetail {
    pub fn new(root: f64, point: Vec3) -> HitDetail {
        HitDetail {
            t: root,
            point: point,
            normal: Vec3::zero(),
            front_face: false,
            calc: false,
        }
    }

    pub fn calc_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        self.calc = true;
    }

    pub fn normal(&self) -> Vec3 {
        if self.calc == false { panic!("face normal not calculated!") }
        self.normal
    }
}
