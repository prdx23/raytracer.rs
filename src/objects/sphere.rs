use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect};

use crate::materials::Material;


#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: usize,
}

impl Intersect for Sphere {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().sq_len();
        let half_b = oc.dot(ray.direction());
        let c = oc.sq_len() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None
            }
        }

        Some(root)
    }

    fn get_intersect_normal(&self, ray: &Ray, t: f64) -> Vec3 {
        let point = ray.at(t);
        let outward_normal = (point - self.center).unit();
        outward_normal
    }

    fn material<'a>(&self, materials: &'a Vec<Material>) -> &'a Material {
        &materials[self.material]
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
