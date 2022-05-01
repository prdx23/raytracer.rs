use std::fmt;
use std::cmp::Ordering;

use crate::Ray;
use crate::behaviors::{ Intersect, IntersectResult };
use crate::objects::{ Aabb, NullObject };


#[derive(Debug)]
pub struct BvhNode {
    pub left: Box<dyn Intersect>,
    pub right: Box<dyn Intersect>,
    pub bbox: Aabb,
}


impl BvhNode {

    pub fn construct(mut objects: Vec<Box<dyn Intersect>>, axis: usize) -> Self {
        let new_axis = (axis + 1) % 3;
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
                if let Some(objs) = left.subdivide(axis) {
                    return BvhNode::construct(objs, new_axis);
                }
                Self {
                    bbox: left.bbox(),
                    left: left,
                    right: Box::new(NullObject),
                }
            },
            2 => {
                let mut right = objects.pop().unwrap();
                if let Some(objs) = right.subdivide(axis) {
                    right = Box::new(BvhNode::construct(objs, new_axis));
                }

                let mut left = objects.pop().unwrap();
                if let Some(objs) = left.subdivide(axis) {
                    left = Box::new(BvhNode::construct(objs, new_axis));
                }

                Self {
                    bbox: left.bbox().merge(right.bbox()),
                    left, right,
                }
            },
            len => {
                match axis {
                    0 => objects.sort_unstable_by(compare_x),
                    1 => objects.sort_unstable_by(compare_y),
                    _ => objects.sort_unstable_by(compare_z),
                }

                let right = BvhNode::construct(objects.split_off(len / 2), new_axis);
                let left = BvhNode::construct(objects, new_axis);

                Self {
                    bbox: left.bbox().merge(right.bbox()),
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
    obj1.bbox().lower[axis].partial_cmp(&obj2.bbox().lower[axis]).unwrap()
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
        let bbox_res_left = self.left.bbox().intersect(ray, t_min, t_max);
        let bbox_res_right = self.right.bbox().intersect(ray, t_min, t_max);

        match (bbox_res_left, bbox_res_right) {

            (None, None) => {
                return None
            },

            (Some(_), None) => {
                return self.left.intersect(ray, t_min, t_max)
            }

            (None, Some(_)) => {
                return self.right.intersect(ray, t_min, t_max)
            }

            (Some(t_left), Some(t_right)) => {
                let first: &Box<dyn Intersect>;
                let last: &Box<dyn Intersect>;
                if t_left < t_right {
                    first = &self.left;
                    last = &self.right;
                } else {
                    first = &self.right;
                    last = &self.left;
                }
                if let Some(first_result) = first.intersect(ray, t_min, t_max) {
                    if let Some(last_result) = last.intersect(ray, t_min, first_result.t) {
                        return Some(last_result)
                    }
                    return Some(first_result)
                }
                return last.intersect(ray, t_min, t_max)
            }
        }
    }

    fn bbox(&self) -> Aabb {
        Aabb { lower: self.bbox.lower, upper: self.bbox.upper }
    }

    fn subdivide(&self, _: usize) -> Option<Vec<Box<dyn Intersect>>> {
        None
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }

}
