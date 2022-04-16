// use std::fmt;

use crate::Vec3;
use crate::Ray;
// use crate::objects::{Hit, HitDetail};

use crate::objects::{Hit, HitDetail};


// #[derive(Debug, Clone, Copy)]
pub struct World {
    objects: Vec<Box<dyn Hit>>,
}


impl World {

    pub fn new() -> World {
        World { objects: Vec::new() }
    }

    // pub fn objects(&self) -> &Vec<Box<dyn Hit>> {
    //     &self.objects
    // }

    pub fn add(&mut self, object: impl Hit + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hit for World {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitDetail> {

        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut final_detail = HitDetail::new(0.0, Vec3::zero());

        for object in self.objects.iter() {
            if let Some(detail) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = detail.t;
                final_detail = detail;
            }
        }

        return match hit_anything {
            true => Some(final_detail),
            false => None,
        }
    }
}

// impl fmt::Debug for World {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_list().entries(self.objects.iter()).finish()
//     }
// }
