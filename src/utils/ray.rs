

use crate::Vec3;


// #[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    invd_cache: Vec3,
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        unsafe { crate::RAY_COUNT += 1; }
        Ray { 
            origin: origin,
            direction: direction,
            invd_cache: Vec3::new(
                1.0 / direction.x,
                1.0 / direction.y,
                1.0 / direction.z,
            ),
        }
    }

    pub fn origin(&self) -> Vec3 { self.origin }

    pub fn direction(&self) -> Vec3 { self.direction }

    pub fn invd_cache(&self) -> Vec3 { self.invd_cache }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

