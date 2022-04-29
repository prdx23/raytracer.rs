
use crate::Vec3;
use crate::Ray;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};

use rand::Rng;

use crate::materials::Material;


#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}


impl Dielectric {
    pub fn new(ir: f64) -> Material {
        Self { refraction_index: ir }.into()
    }
}


impl Scatter for Dielectric {

    fn scatter(&self, ray: &Ray, result: IntersectResult) -> Option<ScatterResult> {

        let refraction_ratio = match result.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let ray_dir = ray.direction().unit();

        let cos_theta = (-ray_dir).dot(result.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let schlick_approx = reflectance(cos_theta, refraction_ratio);
        let mut rng = rand::thread_rng();

        let dir = match cannot_refract || (schlick_approx > rng.gen::<f64>()) {
            true => Vec3::reflect(ray_dir, result.normal),
            false => Vec3::refract(ray_dir, result.normal, refraction_ratio),
        };

        // let reflected_ray = Ray { origin: result.point, direction: dir };
        Some(ScatterResult {
            // ray: Ray { origin: result.point, direction: dir },
            ray: Ray::new(result.point, dir),
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        })
    }

}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_sq = r0 * r0;
    r0_sq + (1.0 - r0_sq) * (1.0 - cosine).powf(5.0)
}
