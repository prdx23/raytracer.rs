use core::f64;
use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::{ Aabb, Triangle };


#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub index_amt: usize,
    // normal: Vec3,
    pub material: usize,
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

    fn intersect(&self, _: &Ray, _: f64, _: f64) -> Option<IntersectResult> {
        panic!("Intersection check on undivided Mesh!");
    }

    fn bbox(&self) -> Aabb {
        let mut lower = Vec3::inf();
        let mut upper = Vec3::neg_inf();

        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex[i] < lower[i] { lower[i] = vertex[i] }
                if vertex[i] > upper[i] { upper[i] = vertex[i] }
            }
        }

        Aabb { lower, upper }
    }

    fn divide(&self) -> Option<Vec<Box<dyn Intersect>>> {
        let mut triangles: Vec<Box<dyn Intersect>> = Vec::with_capacity(
            self.index_amt / 3
        );

        let amt = self.index_amt;
        let parent_mesh = Rc::new(self.clone());
        for i in (0..amt).step_by(3) {
            triangles.push(Box::new(
                Triangle {
                    mesh: Rc::clone(&parent_mesh),
                    offset: i
                }
            ));
        }

        Some(triangles)
    }


    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Mesh [vertices:{} indices:{} material:{}]>",
            self.vertices.len(),
            self.index_amt / 3,
            self.material,
        )
    }

}
