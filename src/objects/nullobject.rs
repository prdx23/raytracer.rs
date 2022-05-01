use std::fmt;

use crate::Ray;
use crate::behaviors::{ Intersect, IntersectResult };
use crate::objects::Aabb;



pub struct NullObject;



impl Intersect for NullObject {

    fn intersect(&self, _: &Ray, _: f64, _: f64) -> Option<IntersectResult> {
        None
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::null()
    }

    fn subdivide(&self) -> Option<Vec<Box<dyn Intersect>>> {
        None
    }

    fn repr(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<NullObject>")
    }

}
