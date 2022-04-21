
use crate::Ray;
use crate::behaviors::{Intersect};
// use crate::behaviors::{Intersect, Scatter};
// use crate::behaviors::{IntersectResult, ScatterResult};


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

impl World {

    pub fn find_intersection(&self, ray: &Ray, t_min: f64, t_max: f64)
        -> Option<(usize, f64)>
    {

        let mut hit_anything = false;
        let mut closest_t = t_max;
        let mut closest_obj_index = 0;

        for (i, object) in self.objects.iter().enumerate() {
            if let Some(t) = object.intersect(ray, t_min, closest_t) {
                hit_anything = true;
                closest_t = t;
                closest_obj_index = i;
            }
        }
        if !hit_anything { return None }

        Some((closest_obj_index, closest_t))
    }
}
