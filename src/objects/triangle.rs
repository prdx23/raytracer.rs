
use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect};

use crate::materials::Material;


#[derive(Debug)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    pub normal: Vec3,
    pub dist_from_origin: f64,
    pub doublesided: bool,

    pub material: usize,
}

impl Triangle {

    pub fn new(
        v0: Vec3, v1: Vec3, v2: Vec3, material: usize, doublesided: bool
    ) -> Self {
        let a = v1 - v0;
        let b = v2 - v0;
        let normal = a.cross(b);
        let dist = -(normal.dot(v0));

        Self {
            v0, v1, v2, material, doublesided,
            normal: normal,
            dist_from_origin: dist,
        }
    }

}



impl Intersect for Triangle {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {

        let normal = self.normal.unit();

        // backface culling
        if ray.direction.dot(normal) > 0.0 && !self.doublesided {
            return None
        }

        // check if ray parallel to plane
        let normal_dot_raydir = normal.dot(ray.direction);
        if normal_dot_raydir.abs() < f64::EPSILON {
            return None
        }

        // calc ray intersection with plane
        let normal_dot_rayorigin = normal.dot(ray.origin);
        let d = self.dist_from_origin;
        let t = -(normal_dot_rayorigin + d) / normal_dot_raydir;

        // triangle is behind ray
        if t < 0.0 { return None }

        // closer obj already found
        if t < t_min || t > t_max { return None }

        // inside-outside test
        let point = ray.at(t);
        let mut c: Vec3;

        c = (self.v1 - self.v0).cross(point - self.v0);
        if normal.dot(c) < 0.0 { return None }

        c = (self.v2 - self.v1).cross(point - self.v1);
        if normal.dot(c) < 0.0 { return None }

        c = (self.v0 - self.v2).cross(point - self.v2);
        if normal.dot(c) < 0.0 { return None }

        Some(t)
    }

    fn get_intersect_normal(&self, _: &Ray, _: f64) -> Vec3 {
        // let a = self.v1 - self.v0;
        // let b = self.v2 - self.v0;
        // a.cross(b).unit()
        self.normal
    }

    fn material<'a>(&self, materials: &'a Vec<Material>) -> &'a Material {
        &materials[self.material]
    }
}
