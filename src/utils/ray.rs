

use crate::Vec3;


// #[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {

    // pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    //     Ray { origin: origin, direction: direction }
    // }

    pub fn origin(&self) -> Vec3 { self.origin }

    pub fn direction(&self) -> Vec3 { self.direction }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

