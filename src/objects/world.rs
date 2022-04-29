use std::fmt;

use crate::{Ray, Vec3};
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::Aabb;


#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Intersect>>,
}


impl World {

    pub fn new() -> World {
        World { objects: Vec::new() }
    }

    pub fn add(&mut self, object: impl Intersect + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Intersect for World {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64)
        -> Option<IntersectResult>
    {
        let mut hit_anything = false;
        let mut closest_t = t_max;
        let mut closest_t_result = IntersectResult::new(
            &ray, closest_t, Vec3::zero(), 0
        );

        for object in self.objects.iter() {
            if object.bounding_box().intersect(ray, t_min, closest_t) {
                if let Some(result) = object.intersect(ray, t_min, closest_t) {
                    hit_anything = true;
                    closest_t = result.t;
                    closest_t_result = result;
                }
            }
        }
        if !hit_anything { return None }

        Some(closest_t_result)
    }

    fn bounding_box(&self) -> Aabb {
        let mut bb = Aabb::null();
        for object in self.objects.iter() {
            bb = bb.merge(object.bounding_box());
        }
        bb
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
