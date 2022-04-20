use std::fmt;
use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::Color;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};


#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}


impl Metal {

    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color.normalize(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }

    pub fn grey(fuzz: f64) -> Self {
        Self {
            albedo: Vec3::new(0.5, 0.5, 0.5),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}


impl Scatter for Metal {

    fn rc(self) -> Rc<dyn Scatter> { Rc::new(self) }

    fn scatter(&self, ray: &Ray, result: IntersectResult) -> Option<ScatterResult> {

        let reflect_dir = Vec3::reflect(ray.direction.unit(), result.normal);

        // hemisphere diffusion
        // let rnd_vector = Vec3::random_in_hemisphere(result.normal);
        // let reflect_dir = relfect_dir + (self.fuzz * rnd_vector);

        // random in sphere approx diffusion
        let rnd_vector = Vec3::random_in_unit_sphere().unit();
        let reflect_dir = reflect_dir + (self.fuzz * (rnd_vector + result.normal));

        let reflected_ray = Ray {
            origin: result.point,
            direction: reflect_dir,
        };

        match reflected_ray.direction.dot(result.normal) > 0.0 {
            true => Some(ScatterResult {
                ray: reflected_ray,
                attenuation: self.albedo,
            }),
            false => None,
        }
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
