use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::objects::Aabb;

// use enum_dispatch::enum_dispatch;

// #[enum_dispatch(Object)]
pub trait Intersect {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectResult>;
    fn bbox(&self) -> Aabb;
    fn divide(&self) -> Option<Vec<Box<dyn Intersect>>>;
    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl fmt::Debug for dyn Intersect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.repr(f)
    }
}



#[derive(Debug, Clone)]
pub struct IntersectResult {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: usize,
}

impl IntersectResult {
    pub fn new(ray: &Ray, t: f64, outward_normal: Vec3, mat: usize ) -> Self {
        let point = ray.at(t);
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        IntersectResult { t, point, normal, front_face, material: mat }
    }
}
