use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::IntersectResult;


pub trait Scatter {
    fn rc(self) -> Rc<dyn Scatter>;
    fn scatter(&self, result: IntersectResult) -> Option<ScatterResult>;
    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl fmt::Debug for dyn Scatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.repr(f)
    }
}

#[derive(Debug, Clone)]
pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Vec3,
}

// impl ScatterResult {
//     pub fn new(ray: Ray, attn: Vec3) -> Self {
//         ScatterResult { ray: ray, attenuation: attn, }
//     }
// }
