pub mod sphere;
pub mod mesh;
pub mod world;
pub mod aabb;
pub mod bvh;
pub mod helpers;
pub mod nullobject;

pub use sphere::Sphere;
pub use mesh::Mesh;
pub use aabb::Aabb;
pub use bvh::BvhNode;
// pub use nullobject::NullObject;
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
