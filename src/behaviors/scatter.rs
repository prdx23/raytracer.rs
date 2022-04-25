use enum_dispatch::enum_dispatch;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::IntersectResult;


#[enum_dispatch(Material)]
pub trait Scatter {
    fn scatter(&self, ray: &Ray, result: IntersectResult) -> Option<ScatterResult>;
    fn emit(&self) -> Vec3 { Vec3::zero() }
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
