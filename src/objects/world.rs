
use crate::{Ray, Vec3};
use crate::behaviors::{Intersect, IntersectResult};


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

    pub fn find_intersection(&self, ray: &Ray)
        -> Option<(usize, IntersectResult)>
    {
        let t_min = 0.0001;
        let mut hit_anything = false;
        let mut closest_t = f64::INFINITY;
        let mut closest_obj_index = 0;
        let mut closest_t_result = IntersectResult::new(
            &ray, closest_t, Vec3::zero()
        );

        for (i, object) in self.objects.iter().enumerate() {
            if let Some(result) = object.intersect(ray, t_min, closest_t) {
                hit_anything = true;
                closest_t = result.t;
                closest_t_result = result;
                closest_obj_index = i;
            }
        }
        if !hit_anything { return None }

        Some((closest_obj_index, closest_t_result))
    }
}
