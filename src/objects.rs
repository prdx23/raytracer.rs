pub mod sphere;
pub mod triangle;
pub mod world;

pub use sphere::Sphere;
pub use triangle::Triangle;
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
