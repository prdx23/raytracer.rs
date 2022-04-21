use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Scatter};


pub trait Intersect {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64>;

    fn get_intersect_normal(&self, ray: &Ray, t: f64) -> Vec3;

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn material(&self) -> &Rc<dyn Scatter>;
}

impl fmt::Debug for dyn Intersect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.repr(f)
    }
}



#[derive(Debug, Clone)]
pub struct IntersectResult {
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl IntersectResult {
    pub fn new(point: Vec3, ray: &Ray, outward_normal: Vec3 ) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        IntersectResult {
            point: point, normal: normal, front_face: front_face,
        }
    }
}
