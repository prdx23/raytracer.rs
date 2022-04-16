
use crate::Vec3;
use crate::Ray;
use crate::objects::{Hit, HitDetail};


// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    // pub fn new(center: Vec3, radius: f64) -> Sphere {
    //     Sphere { center: center, radius: radius }
    // }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitDetail> {
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

        // Note: can optimize 
        // only calc normal after nearest is found

        let point = ray.at(root);
        let outward_normal = (point - self.center).unit();

        let mut detail = HitDetail::new(root, point);
        detail.calc_face_normal(ray, outward_normal);

        Some(detail)
    }
}
