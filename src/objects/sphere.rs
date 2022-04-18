use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::behaviors::{Scatter, ScatterResult};


// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Scatter>,
}

// impl Sphere {
//     pub fn new(center: Vec3, radius: f64) -> Sphere {
//         Sphere { center: center, radius: radius }
//     }
// }

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

    fn get_intersect_result(&self, ray: &Ray, t: f64)
        -> Option<ScatterResult>
    {
        let point = ray.at(t);
        let outward_normal = (point - self.center).unit();
        let result = IntersectResult::new(point, &ray, outward_normal);
        self.material.scatter(result)
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
