

use crate::{ Vec3, Ray };
// use crate::behaviors::{Intersect, IntersectResult};


#[derive(Debug, Clone)]
pub struct Aabb {
    pub lower: Vec3,
    pub upper: Vec3,
}


impl Aabb {

    pub fn intersect(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> Option<f64> {
        unsafe { crate::INTERSECT_TESTS_AABB += 1; }
        for i in 0..3 {
            let invd = ray.invd_cache()[i];
            let mut t0 = (self.lower[i] - ray.origin()[i]) * invd;
            let mut t1 = (self.upper[i] - ray.origin()[i]) * invd;

            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None
            }
        }
        unsafe { crate::INTERSECT_PASSES_AABB += 1; }
        return Some(t_min)
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            lower: Vec3::new(
                self.lower.x.min(other.lower.x),
                self.lower.y.min(other.lower.y),
                self.lower.z.min(other.lower.z),
            ),
            upper: Vec3::new(
                self.upper.x.max(other.upper.x),
                self.upper.y.max(other.upper.y),
                self.upper.z.max(other.upper.z),
            ),
        }
    }

    pub fn null() -> Self {
        Self { lower: Vec3::zero(), upper: Vec3::zero() }
    }

}
