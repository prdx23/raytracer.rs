
use crate::Ray;
use crate::behaviors::{ Intersect, IntersectResult };
use crate::objects::{ Aabb, Object };


#[derive(Debug)]
pub struct BvhNode {
    pub object: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub bbox: Aabb,
}


const N_BUCKETS: usize = 12;

#[derive(Debug, Default)]
struct Bucket {
    count: usize,
    bbox: Aabb,
}



impl BvhNode {

    pub fn construct(
        mut primitives: Vec<Object>, objects: &mut Vec<Object>,
        nodes: &mut Vec<BvhNode>
    ) -> usize {

        match primitives.len() {
            0 => {
                nodes.push(Self {
                    object: None,
                    left: None,
                    right: None,
                    bbox: Aabb::null(),
                })
            },
            1 => {
                let object = primitives.pop().unwrap();
                let bbox = object.bbox();
                objects.push(object);
                nodes.push(Self {
                    bbox,
                    left: None,
                    right: None,
                    object: Some(objects.len() - 1),
                })
            },
            2 => {
                let right = primitives.pop().unwrap();
                let left = primitives.pop().unwrap();

                let bbox = left.bbox().clone().merge(right.bbox().clone());
                let left_node = BvhNode::construct(vec![left], objects, nodes);
                let right_node = BvhNode::construct(vec![right], objects, nodes);

                nodes.push(Self {
                    bbox,
                    left: Some(left_node),
                    right: Some(right_node),
                    object: None,
                });
            },
            len => {

                // calc combined obj aab and obj centroid aabb
                let mut full_bounds: Aabb = Default::default();
                let mut centroid_bounds: Aabb = Default::default();

                for obj in primitives.iter() {
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
                for obj in primitives.iter() {
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
                let mut left_list: Vec<Object> = Vec::with_capacity(len / 2);
                let mut right_list: Vec<Object> = Vec::with_capacity(len / 2);
                for obj in primitives.into_iter() {
                    let offset = centroid_bounds.offset(obj.bbox().centroid());
                    let n = (offset[dim] * N_BUCKETS as f64) as usize;
                    if n <= min_cost_bucket {
                        left_list.push(obj);
                    } else {
                        right_list.push(obj);
                    }
                }

                let left = BvhNode::construct(left_list, objects, nodes);
                let right = BvhNode::construct(right_list, objects, nodes);
                nodes.push(Self {
                    bbox: full_bounds,
                    left: Some(left),
                    right: Some(right),
                    object: None,
                });
            }
        }
        nodes.len() - 1
    }
}


impl BvhNode {

    pub fn intersect(
        &self, ray: &Ray, t_min: f64, t_max: f64, objects: &Box<[Object]>,
        nodes: &Box<[BvhNode]>,
    ) -> Option<IntersectResult> {


        let l: usize;
        let r: usize;

        if let (Some(left_i), Some(right_i)) = (self.left, self.right) {
            l = left_i;
            r = right_i;
        } else {
            return match self.object {
                Some(object) => objects[object].intersect(ray, t_min, t_max),
                None => None,
            }
        }

        debug_assert!(l < nodes.len());
        debug_assert!(r < nodes.len());

        let mut left = unsafe { &nodes.get_unchecked(l) };
        let mut right = unsafe { &nodes.get_unchecked(r) };

        let left_bbox_res = left.bbox.intersect(ray, t_min, t_max);
        let right_bbox_res = right.bbox.intersect(ray, t_min, t_max);

        match (left_bbox_res, right_bbox_res) {

            (None, None) => {
                None
            },

            (Some(_), None) => {
                left.intersect(ray, t_min, t_max, objects, nodes)
            },

            (None, Some(_)) => {
                right.intersect(ray, t_min, t_max, objects, nodes)
            },

            (Some(left_t), Some(right_t)) => {
                if left_t > right_t {
                    // right is closer, so swap to check right first
                    std::mem::swap(&mut left, &mut right);
                }

                match left.intersect(ray, t_min, t_max, objects, nodes) {
                    Some(left_res) => {
                        match right.intersect(ray, t_min, left_res.t, objects, nodes) {
                            Some(right_res) => return Some(right_res),
                            None => return Some(left_res),
                        }
                    },
                    None => {
                        return right.intersect(ray, t_min, t_max, objects, nodes)
                    },
                }
            }

        }
    }
}
