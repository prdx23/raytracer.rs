use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::{ Aabb, Mesh };


#[derive(Debug)]
pub struct Triangle {
    pub mesh: Rc<Mesh>,
    pub offset: usize,
}


impl Triangle {

    pub fn vertices(&self) -> [Vec3; 3] {
        [
            self.mesh.vertices[self.mesh.indices[self.offset + 0]],
            self.mesh.vertices[self.mesh.indices[self.offset + 1]],
            self.mesh.vertices[self.mesh.indices[self.offset + 2]],
        ]
    }
}


impl Intersect for Triangle {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) 
        -> Option<IntersectResult>
    {
        let [v0, v1, v2] = self.vertices();
        let result = ray_triangle_intersect(
            v0, v1, v2, false, ray, t_min, t_max
        );

        if let Some(t) = result {
            let normal = (v1 - v0).cross(v2 - v0).unit();
            return Some(IntersectResult::new(ray, t, normal, self.mesh.material))
        }
        None
    }

    fn bbox(&self) -> Aabb {
        let mut lower = Vec3::inf();
        let mut upper = Vec3::neg_inf();

        for vertex in self.vertices() {
            for i in 0..3 {
                if vertex[i] < lower[i] { lower[i] = vertex[i] }
                if vertex[i] > upper[i] { upper[i] = vertex[i] }
            }
        }

        Aabb { lower, upper }
    }

    fn divide(&self) -> Option<Vec<Box<dyn Intersect>>> {
        None
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Triangle [ mesh: ")?;
        self.mesh.repr(f)?;
        write!(f, ", offset: {} ]>", self.offset)
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
