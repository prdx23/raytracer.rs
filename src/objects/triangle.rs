use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect};

use crate::materials::Material;


#[derive(Debug)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub doublesided: bool,
    pub material: usize,
}


impl Intersect for Triangle {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        // moller-trumbore ray-triangle intersection algo

        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        if !self.doublesided {
            // backface culling
            if det < f64::EPSILON { return None }
        } else {
            // ray and triangle are parallel if det ~= 0
            if det.abs() < f64::EPSILON { return None }
        };

        // cramers rule solution to t-u-v cords from x-y-z
        let invdet = 1.0 / det;

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(pvec) * invdet;
        if u < 0.0 || u > 1.0 { return None }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * invdet;
        if v < 0.0 || u + v > 1.0 { return None }

        let t = v0v2.dot(qvec) * invdet;

        // closer obj already found
        if t < t_min || t > t_max { return None }

        Some(t)
    }

    fn get_intersect_normal(&self, _: &Ray, _: f64) -> Vec3 {
        let a = self.v1 - self.v0;
        let b = self.v2 - self.v0;
        a.cross(b).unit()
    }

    fn material<'a>(&self, materials: &'a Vec<Material>) -> &'a Material {
        &materials[self.material]
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
