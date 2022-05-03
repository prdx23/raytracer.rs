use std::fmt;

use crate::{Ray, Vec3};
use crate::behaviors::{Intersect, IntersectResult};
use crate::objects::{ Aabb, NullObject };


pub const N_OBJLIST: usize = 4;


#[derive(Debug)]
pub struct ObjectList {
    pub objects: [Box<dyn Intersect>; N_OBJLIST],
}


impl ObjectList {

    pub fn new(mut objects: Vec<Box<dyn Intersect>>) -> Self {
        if objects.len() > 4 {
            panic!("ObjectList does not support more than 4 objects");
        }

        let mut array: [Box<dyn Intersect>; N_OBJLIST] = [
            Box::new(NullObject{}), Box::new(NullObject{}),
            Box::new(NullObject{}), Box::new(NullObject{}),
        ];

        for i in 0..4 {
            if let Some(obj) = objects.pop() {
                array[i] = obj
            }
        }
        Self { objects: array }
    }

}

impl Intersect for ObjectList {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64)
        -> Option<IntersectResult>
    {
        let mut hit_anything = false;
        let mut closest_t = t_max;
        let mut closest_t_result = IntersectResult::new(
            &ray, closest_t, Vec3::zero(), 0
        );

        for object in self.objects.iter() {
            if let Some(result) = object.intersect(ray, t_min, closest_t) {
                hit_anything = true;
                closest_t = result.t;
                closest_t_result = result;
            }
        }
        if !hit_anything { return None }

        Some(closest_t_result)
    }

    fn bbox(&self) -> Aabb {
        let mut bb = Aabb::null();
        for object in self.objects.iter() {
            bb = bb.merge(object.bbox());
        }
        bb
    }

    fn divide(&self) -> Option<Vec<Box<dyn Intersect>>> {
        None
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ObjectList [{:?}]>", self.objects)
    }
}
