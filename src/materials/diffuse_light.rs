use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::Color;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};


#[derive(Debug, Clone)]
pub struct DiffuseLight {
    albedo: Vec3,
}


impl DiffuseLight {

    pub fn new(color: Color, intensity: f64) -> Self {
        Self {
            albedo: color.normalize() * intensity,
        }
    }

    pub fn white(intensity: f64) -> Self {
        Self {
            albedo: Vec3::new(1.0, 1.0, 1.0) * intensity,
        }
    }
}


impl Scatter for DiffuseLight {

    fn rc(self) -> Rc<dyn Scatter> { Rc::new(self) }

    fn scatter(&self, _: &Ray, _: IntersectResult) -> Option<ScatterResult> {
        None
    }

    fn emit(&self) -> Vec3 {
        self.albedo
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
