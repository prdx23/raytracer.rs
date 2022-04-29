use core::f64;
use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::Aabb;


#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
    faces: usize,
    normal: Vec3,
    material: usize,
}

impl Mesh {

    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, mat: usize) -> Self {
        Self {
            faces: indices.len(),
            vertices, indices,
            material: mat,
            normal: Vec3::zero(),
        }
    }

}


impl Intersect for Mesh {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) 
        -> Option<IntersectResult>
    {

        if self.faces % 3 != 0 {
            panic!("not enough indices for triangle in mesh");
        }

        let mut v0 = Vec3::zero();
        let mut v1 = Vec3::zero();
        let mut v2 = Vec3::zero();
        let mut hit_anything = false;
        let mut closest_t = t_max;

        for i in (0..self.faces).step_by(3) {
            v0 = self.vertices[self.indices[i + 0]];
            v1 = self.vertices[self.indices[i + 1]];
            v2 = self.vertices[self.indices[i + 2]];

            let result = ray_triangle_intersect(
                v0, v1, v2, false, ray, t_min, t_max
            );

            if let Some(t) = result {
                if t > t_min && t < closest_t {
                    hit_anything = true;
                    closest_t = t;
                }
            }
        }
        if !hit_anything { return None }

        let normal = (v1 - v0).cross(v2 - v0).unit();
        Some(IntersectResult::new(ray, closest_t, normal, self.material))
    }

    fn bounding_box(&self) -> Aabb {
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex[i] < min[i] { min[i] = vertex[i] }
                if vertex[i] > max[i] { max[i] = vertex[i] }
            }
        }

        Aabb { min, max }
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "<Mesh [vertices:{} faces:{} material:{}]>",
            self.vertices.len(),
            self.faces / 3,
            self.material,
        )
    }

}


pub fn ray_triangle_intersect(
    v0: Vec3, v1: Vec3, v2: Vec3, doublesided: bool,
    ray: &Ray, _: f64, _: f64
) -> Option<f64> {

    // moller-trumbore ray-triangle intersection algo

    unsafe { crate::INTERSECT_TESTS += 1; }

    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let pvec = ray.direction().cross(v0v2);
    let det = v0v1.dot(pvec);

    if !doublesided {
        // backface culling
        if det < f64::EPSILON { return None }
    } else {
        // ray and triangle are parallel if det ~= 0
        if det.abs() < f64::EPSILON { return None }
    };

    // cramers rule solution to t-u-v cords from x-y-z
    let invdet = 1.0 / det;

    let tvec = ray.origin() - v0;
    let u = tvec.dot(pvec) * invdet;
    if u < 0.0 || u > 1.0 { return None }

    let qvec = tvec.cross(v0v1);
    let v = ray.direction().dot(qvec) * invdet;
    if v < 0.0 || u + v > 1.0 { return None }

    let t = v0v2.dot(qvec) * invdet;

    // closer obj already found
    // if t < t_min || t > t_max { return None }

    unsafe { crate::INTERSECT_PASSES += 1; }
    Some(t)
}
