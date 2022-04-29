

use crate::{ Vec3, Ray };
// use crate::behaviors::{Intersect, IntersectResult};


#[derive(Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}


impl Aabb {

    pub fn intersect(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        unsafe { crate::INTERSECT_TESTS_AABB += 1; }
        for i in 0..3 {
            let invd = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * invd;
            let mut t1 = (self.max[i] - ray.origin()[i]) * invd;

            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false
            }
        }
        unsafe { crate::INTERSECT_PASSES_AABB += 1; }
        return true
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn null() -> Self {
        Self { min: Vec3::zero(), max: Vec3::zero() }
    }

}
