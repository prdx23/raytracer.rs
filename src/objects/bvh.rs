use std::rc::Rc;

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


const N_BUCKETS: usize = 12;

#[derive(Debug, Default)]
struct Bucket {
    count: usize,
    bbox: Aabb,
}



impl BvhNode {

    pub fn construct(mut objects: Vec<Box<dyn Intersect>>) -> Self {
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
                let bbox = object.bbox();
                Self {
                    bbox,
                    left: None,
                    right: None,
                    object: Some(object),
                }
            },
            2 => {
                let right = objects.pop().unwrap();
                let left = objects.pop().unwrap();

                let bbox = left.bbox().clone().merge(right.bbox().clone());
                let left_node = BvhNode::construct(vec![left]);
                let right_node = BvhNode::construct(vec![right]);

                Self {
                    bbox,
                    left: Some(Box::new(left_node)),
                    right: Some(Box::new(right_node)),
                    object: None,
                }
            },
            len => {

                // calc combined obj aab and obj centroid aabb
                let mut full_bounds: Aabb = Default::default();
                let mut centroid_bounds: Aabb = Default::default();

                for obj in objects.iter() {
                    centroid_bounds = centroid_bounds.merge(
                        Aabb {
                            lower: obj.bbox().centroid(),
                            upper: obj.bbox().centroid(),
                        }
                    );
                    full_bounds = full_bounds.merge(obj.bbox().clone());
                }

                // pick dimention as longest centroid aabb span
                let dim = centroid_bounds.max_extent();

                // create N buckets and put each centroid into its bucket
                let mut buckets: [Bucket; N_BUCKETS] = Default::default();
                for obj in objects.iter() {
                    let offset = centroid_bounds.offset(obj.bbox().centroid());
                    let mut n = (offset[dim] * N_BUCKETS as f64) as usize;
                    if n == N_BUCKETS { n -= 1 }
                    buckets[n].count += 1;
                    buckets[n].bbox = buckets[n].bbox.clone().merge(obj.bbox());
                }


                // for each bucket, calc area of all buckets before and after it
                let mut costs: [f64; N_BUCKETS - 1] = Default::default();
                for i in 0..(N_BUCKETS - 1) {
                    let mut b0: Aabb = Default::default();
                    let mut b1: Aabb = Default::default();
                    let mut count0: usize = 0;
                    let mut count1: usize = 0;

                    for j in 0..(i + 1) {
                        b0 = b0.merge(buckets[j].bbox.clone());
                        count0 += buckets[j].count;
                    }

                    for j in (i + 1)..N_BUCKETS {
                        b1 = b1.merge(buckets[j].bbox.clone());
                        count1 += buckets[j].count;
                    }

                    let area0 = count0 as f64 * b0.area();
                    let area1 = count1 as f64 * b1.area();
                    costs[i] = 0.125 + (area0 + area1) / full_bounds.area();
                }

                // pick the bucket split that minimizes area
                let mut min_cost = costs[0];
                let mut min_cost_bucket: usize = 0;
                for i in 0..(N_BUCKETS - 1) {
                    if costs[i] < min_cost {
                        min_cost = costs[i];
                        min_cost_bucket = i;
                    }
                }

                // divide objs into 2 using centroids with bucket split
                let mut left_list: Vec<Box<dyn Intersect>> = Vec::with_capacity(len / 2);
                let mut right_list: Vec<Box<dyn Intersect>> = Vec::with_capacity(len / 2);
                for obj in objects.into_iter() {
                    let offset = centroid_bounds.offset(obj.bbox().centroid());
                    let n = (offset[dim] * N_BUCKETS as f64) as usize;
                    if n <= min_cost_bucket {
                        left_list.push(obj);
                    } else {
                        right_list.push(obj);
                    }
                }

                Self {
                    bbox: full_bounds,
                    left: Some(Box::new(BvhNode::construct(left_list))),
                    right: Some(Box::new(BvhNode::construct(right_list))),
                    object: None,
                }
            }
        }
    }
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
