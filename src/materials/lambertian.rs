use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};


#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}


impl Lambertian {
    pub fn grey() -> Self {
        Self {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}


impl Scatter for Lambertian {

    fn rc(self) -> Rc<dyn Scatter> { Rc::new(self) }

    fn scatter(&self, result: IntersectResult) -> Option<ScatterResult> {

        let rnd_vector = Vec3::random_in_hemisphere(result.normal);
        let reflected_ray = Ray {
            origin: result.point,
            direction: result.normal + rnd_vector,
        };

        Some(ScatterResult {
            ray: reflected_ray,
            attenuation: self.albedo,
        })
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
