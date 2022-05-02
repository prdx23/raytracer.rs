use std::cmp::Ordering;

use crate::Ray;
use crate::behaviors::{ Intersect, IntersectResult };
use crate::objects::{ Aabb };


#[derive(Debug)]
pub struct BvhNode {
    pub object: Option<Box<dyn Intersect>>,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub bbox: Aabb,
}


impl BvhNode {

    pub fn construct(mut objects: Vec<Box<dyn Intersect>>, axis: usize) -> Self {
        let new_axis = (axis + 1) % 3;
        match objects.len() {
            0 => {
                Self {
                    object: None,
                    left: None,
                    right: None,
                    bbox: Aabb::null(),
                }
            },
            1 => {
                let object = objects.pop().unwrap();
                if let Some(objs) = object.subdivide(axis) {
                    return BvhNode::construct(objs, new_axis);
                }
                Self {
                    bbox: object.bbox(),
                    left: None,
                    right: None,
                    object: Some(object),
                }
            },
            // 2 => {
            //     let mut right = objects.pop().unwrap();
            //     if let Some(objs) = right.subdivide(axis) {
            //         right = Box::new(BvhNode::construct(objs, new_axis));
            //     }

            //     let mut left = objects.pop().unwrap();
            //     if let Some(objs) = left.subdivide(axis) {
            //         left = Box::new(BvhNode::construct(objs, new_axis));
            //     }

            //     Self {
            //         bbox: left.bbox().merge(right.bbox()),
            //         left, right,
            //     }
            // },
            len => {
                match axis {
                    0 => objects.sort_unstable_by(compare_x),
                    1 => objects.sort_unstable_by(compare_y),
                    _ => objects.sort_unstable_by(compare_z),
                }

                let right = BvhNode::construct(objects.split_off(len / 2), new_axis);
                let left = BvhNode::construct(objects, new_axis);

                Self {
                    bbox: left.bbox.clone().merge(right.bbox.clone()),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    object: None,
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


impl BvhNode {

    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64)
        -> Option<IntersectResult>
    {

        let mut left: &Box<BvhNode>;
        let mut right: &Box<BvhNode>;

        match (self.left.as_ref(), self.right.as_ref()) {

            // leaf node
            (None, None) => {
                return match &self.object {
                    Some(object) => object.intersect(ray, t_min, t_max),
                    None => None,
                }
            }

            (Some(l), Some(r)) => {
                left = l;
                right = r;
            }

            _ => {
                panic!("One node of BvhNode is None while other is not");
            }

        }


        match left.bbox.intersect(ray, t_min, t_max) {

            Some(left_t) => {
                match right.bbox.intersect(ray, t_min, t_max) {
                    Some(right_t) => {

                        if left_t > right_t {
                            // right is closer, so swap to check right first
                            std::mem::swap(&mut left, &mut right);
                        }

                        match left.intersect(ray, t_min, t_max) {
                            Some(left_res) => {
                                match right.intersect(ray, t_min, left_res.t) {
                                    Some(right_res) => return Some(right_res),
                                    None => return Some(left_res),
                                }
                            },
                            None => {
                                return right.intersect(ray, t_min, t_max)
                            },
                        }

                    },
                    None => {
                        return left.intersect(ray, t_min, t_max)
                    },
                }
            },

            None => {
                if let Some(_) = right.bbox.intersect(ray, t_min, t_max) {
                    return right.intersect(ray, t_min, t_max)
                }
            },

        }

        None
    }
}
