use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::Aabb;


#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: usize,
}

impl Intersect for Sphere {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) 
        -> Option<IntersectResult>
    {
        unsafe { crate::INTERSECT_TESTS_SP += 1; }

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

        let point = ray.at(root);
        let outward_normal = (point - self.center).unit();

        unsafe { crate::INTERSECT_PASSES_SP += 1; }
        Some(IntersectResult::new(&ray, root, outward_normal, self.material))
    }

    fn bounding_box(&self) -> Aabb {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        Aabb {
            lower: self.center - radius_vector,
            upper: self.center + radius_vector,
        }
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
