use core::f64;
use std::fmt;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::Aabb;

use rand::Rng;

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
    index_amt: usize,
    // normal: Vec3,
    material: usize,
}

impl Mesh {

    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, mat: usize) -> Self {

        let index_amt = indices.len();
        if index_amt % 3 != 0 {
            panic!("not enough indices for triangle in mesh");
        }

        Self {
            index_amt, vertices, indices,
            material: mat,
            // normal: Vec3::zero(),
        }
    }
}


impl Intersect for Mesh {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) 
        -> Option<IntersectResult>
    {
        let mut v0 = Vec3::zero();
        let mut v1 = Vec3::zero();
        let mut v2 = Vec3::zero();
        let mut hit_anything = false;
        let mut closest_t = t_max;

        for i in (0..self.index_amt).step_by(3) {
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
        let mut lower = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut upper = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex[i] < lower[i] { lower[i] = vertex[i] }
                if vertex[i] > upper[i] { upper[i] = vertex[i] }
            }
        }

        Aabb { lower, upper }
    }

    fn subdivide(&self) -> Option<Vec<Box<dyn Intersect>>> {

        // mesh has 20 or less triangles
        if self.index_amt / 3 < 20 { return None }

        let mut left_indices: Vec<usize> = vec![];
        let mut right_indices: Vec<usize> = vec![];

        let mut left_vertices: Vec<Vec3> = vec![];
        let mut right_vertices: Vec<Vec3> = vec![];

        let bbox = self.bounding_box();
        let mid = (bbox.lower + bbox.upper) / 2.0;
        let axis = rand::thread_rng().gen_range(0..3);

        for i in (0..self.index_amt).step_by(3) {
            let v0 = self.vertices[self.indices[i + 0]];
            let v1 = self.vertices[self.indices[i + 1]];
            let v2 = self.vertices[self.indices[i + 2]];

            let centroid = (v0 + v1 + v2) / 3.0;

            if centroid[axis] < mid[axis] {
                update_lists(&mut left_vertices, &mut left_indices, v0);
                update_lists(&mut left_vertices, &mut left_indices, v1);
                update_lists(&mut left_vertices, &mut left_indices, v2);
            } else {
                update_lists(&mut right_vertices, &mut right_indices, v0);
                update_lists(&mut right_vertices, &mut right_indices, v1);
                update_lists(&mut right_vertices, &mut right_indices, v2);
            }
        }

        Some(vec![
            Box::new(Mesh::new(left_vertices, left_indices, self.material)),
            Box::new(Mesh::new(right_vertices, right_indices, self.material)),
        ])
    }


    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Mesh [vertices:{} indices:{} material:{}]>",
            self.vertices.len(),
            self.index_amt / 3,
            self.material,
        )
    }

}

fn update_lists(vertices: &mut Vec<Vec3>, indices: &mut Vec<usize>, v: Vec3) {
    match vertices.iter().position(|&x| x == v) {
        Some(index) => {
            indices.push(index)
        },
        None => {
            indices.push(vertices.len());
            vertices.push(v);
        }
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
