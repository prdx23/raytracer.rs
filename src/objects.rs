pub mod sphere;
pub mod mesh;
pub mod world;
pub mod helpers;

pub use sphere::Sphere;
pub use mesh::Mesh;
pub use world::World;

// use crate::Ray;
// use crate::Vec3;
// use crate::behaviors::{Intersect};
// use crate::materials::Material;

// use enum_dispatch::enum_dispatch;

// #[enum_dispatch]
// #[derive(Debug)]
// pub enum Object {
//     Sphere,
//     Triangle,
// }
