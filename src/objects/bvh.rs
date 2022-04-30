use std::fmt;
use std::cmp::Ordering;

use crate::Ray;
use crate::behaviors::{ Intersect, IntersectResult };
use crate::objects::{ Aabb, NullObject };

use rand::Rng;


#[derive(Debug)]
pub struct BvhNode {
    pub left: Box<dyn Intersect>,
    pub right: Box<dyn Intersect>,
    pub bbox: Aabb,
}


impl BvhNode {

    pub fn construct(mut objects: Vec<Box<dyn Intersect>>) -> Self {
        match objects.len() {
            0 => {
                Self {
                    left: Box::new(NullObject),
                    right: Box::new(NullObject),
                    bbox: Aabb::null(),
                }
            },
            1 => {
                let left = objects.pop().unwrap();
                Self {
                    bbox: left.bounding_box(),
                    left: left,
                    right: Box::new(NullObject),
                }
            },
            // 2 => {
            //     let right = objects.pop().unwrap();
            //     let left = objects.pop().unwrap();
            //     Self {
            //         bbox: left.bounding_box().merge(right.bounding_box()),
            //         left, right,
            //     }
            // },
            len => {
                match rand::thread_rng().gen_range(0..3) {
                    0 => objects.sort_unstable_by(compare_x),
                    1 => objects.sort_unstable_by(compare_y),
                    _ => objects.sort_unstable_by(compare_z),
                }

                let right = BvhNode::construct(objects.split_off(len / 2));
                let left = BvhNode::construct(objects);

                Self {
                    bbox: left.bounding_box().merge(right.bounding_box()),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }
}


fn compare(
    obj1: &Box<dyn Intersect>, obj2: &Box<dyn Intersect>, axis: usize
) -> Ordering {
    obj1.bounding_box().lower[axis].partial_cmp(
        &obj2.bounding_box().lower[axis]
    ).unwrap()
}

fn compare_x(obj1: &Box<dyn Intersect>, obj2: &Box<dyn Intersect>) -> Ordering {
    compare(obj1, obj2, 0)
}

fn compare_y(obj1: &Box<dyn Intersect>, obj2: &Box<dyn Intersect>) -> Ordering {
    compare(obj1, obj2, 1)
}

fn compare_z(obj1: &Box<dyn Intersect>, obj2: &Box<dyn Intersect>) -> Ordering {
    compare(obj1, obj2, 2)
}


impl Intersect for BvhNode {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64)
        -> Option<IntersectResult>
    {
        if !self.bbox.intersect(ray, t_min, t_max) {
            return None
        }

        match self.left.intersect(ray, t_min, t_max) {
            Some(left_result) => {
                match self.right.intersect(ray, t_min, left_result.t) {
                    Some(right_result) => Some(right_result),
                    None => Some(left_result),
                }
            },
            None => self.right.intersect(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self) -> Aabb {
        Aabb { lower: self.bbox.lower, upper: self.bbox.upper }
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }

}
