pub mod sphere;
pub mod mesh;
pub mod triangle;
// pub mod world;
pub mod aabb;
pub mod bvh;
pub mod helpers;
// pub mod nullobject;

pub use sphere::Sphere;
pub use mesh::Mesh;
pub use triangle::Triangle;
pub use aabb::Aabb;
pub use bvh::BvhNode;
// pub use nullobject::NullObject;
// pub use world::World;

use std::fmt;
use crate::Ray;
use crate::behaviors::{Intersect, IntersectResult};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[derive(Debug)]
pub enum Object {
    Sphere,
    Triangle,
    Mesh,
}
