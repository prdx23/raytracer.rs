use core::f64;
use std::fmt;
use std::sync::Arc;

use crate::{ Vec3, Ray, Matrix4 };
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::{ Aabb, Triangle, Object };


#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indexes: Vec<(usize, usize)>,
    pub index_amt: usize,
    // normal: Vec3,
    pub material: usize,
}

impl Mesh {

    pub fn new(
        vertices: Vec<Vec3>, indexes: Vec<(usize, usize)>,
        normals: Vec<Vec3>, mat: usize
    ) -> Self {

        let index_amt = indexes.len();
        if index_amt % 3 != 0 {
            panic!("not enough indices for triangle in mesh");
        }

        Self {
            index_amt, vertices, indexes, normals,
            material: mat,
            // normal: Vec3::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn transform(&mut self, matrix: Matrix4) {
        let normal_matrix = matrix.inverse().transpose();

        for vertex in self.vertices.iter_mut() {
            *vertex = matrix * (*vertex);
        }

        for normal in self.normals.iter_mut() {
            *normal = normal_matrix * (*normal);
        }
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, tx: f64, ty: f64, tz: f64) {
        self.transform(Matrix4::translate(tx, ty, tz));
    }

    #[allow(dead_code)]
    pub fn scale(&mut self, sx: f64, sy: f64, sz: f64) {
        self.transform(Matrix4::scale(sx, sy, sz));
    }

    #[allow(dead_code)]
    pub fn scale_x(&mut self, sx: f64) {
        self.transform(Matrix4::scale(sx, 1.0, 1.0));
    }

    #[allow(dead_code)]
    pub fn scale_y(&mut self, sy: f64) {
        self.transform(Matrix4::scale(1.0, sy, 1.0));
    }

    #[allow(dead_code)]
    pub fn scale_z(&mut self, sz: f64) {
        self.transform(Matrix4::scale(1.0, 1.0, sz));
    }

    #[allow(dead_code)]
    pub fn rotate_x(&mut self, theta: f64) {
        self.transform(Matrix4::rotate_x(theta));
    }

    #[allow(dead_code)]
    pub fn rotate_y(&mut self, theta: f64) {
        self.transform(Matrix4::rotate_y(theta));
    }

    #[allow(dead_code)]
    pub fn rotate_z(&mut self, theta: f64) {
        self.transform(Matrix4::rotate_z(theta));
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

    fn divide(&self) -> Option<Vec<Object>> {
        let mut triangles: Vec<Object> = Vec::with_capacity(
            self.index_amt / 3
        );

        let amt = self.index_amt;
        let parent_mesh = Arc::new(self.clone());
        for i in (0..amt).step_by(3) {
            triangles.push(
                Triangle {
                    mesh: Arc::clone(&parent_mesh),
                    offset: i
                }.into()
            );
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
