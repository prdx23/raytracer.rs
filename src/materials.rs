use enum_dispatch::enum_dispatch;


pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;


// for enum dispatch
use crate::Ray;
use crate::Vec3;
use crate::behaviors::{Scatter, IntersectResult, ScatterResult};


#[enum_dispatch]
#[derive(Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}
