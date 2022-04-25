
use crate::Vec3;
use crate::Ray;
use crate::Color;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};

use crate::materials::Material;

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    albedo: Vec3,
}


impl DiffuseLight {

    pub fn new(color: Color, intensity: f64) -> Material {
        Self {
            albedo: color.normalize() * intensity,
        }.into()
    }

    pub fn white(intensity: f64) -> Material {
        Self {
            albedo: Vec3::new(1.0, 1.0, 1.0) * intensity,
        }.into()
    }
}


impl Scatter for DiffuseLight {

    fn scatter(&self, _: &Ray, _: IntersectResult) -> Option<ScatterResult> {
        None
    }

    fn emit(&self) -> Vec3 {
        self.albedo
    }

}
