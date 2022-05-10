
use crate::Vec3;
use crate::Ray;
use crate::Color;
use crate::behaviors::{Scatter, ScatterResult, IntersectResult};

use crate::materials::Material;


#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}


impl Lambertian {

    pub fn new(color: Color) -> Material {
        Self { albedo: color.normalize() }.into()
    }

    pub fn grey() -> Material {
        Self { albedo: Vec3::new(0.5, 0.5, 0.5) }.into()
    }

}


impl Scatter for Lambertian {

    fn scatter(&self, _: &Ray, result: IntersectResult) -> Option<ScatterResult> {

        // hemisphere diffusion
        // let rnd_vector = Vec3::random_in_hemisphere(result.normal);
        // let scatter_dir = rnd_vector;

        // unit sphere approx diffusion
        let rnd_vector = Vec3::random_in_unit_sphere().unit();
        let scatter_dir = result.normal + rnd_vector;

        let scattered_ray = match scatter_dir.near_zero() {
            true => Ray::new(
                result.point + (crate::BIAS * result.normal),
                result.normal,
            ),
            false => Ray::new(
                result.point + (crate::BIAS * result.normal),
                scatter_dir,
            ),
        };
        // let scattered_ray = match scatter_dir.near_zero() {
        //     true => Ray {
        //         origin: result.point,
        //         direction: result.normal,
        //     },
        //     false => Ray {
        //         origin: result.point,
        //         direction: scatter_dir,
        //     },
        // };

        Some(ScatterResult {
            ray: scattered_ray,
            attenuation: self.albedo,
        })
    }

}
