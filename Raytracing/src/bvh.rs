use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittable: &mut [Rc<dyn Hittable>], time0: f32, time1: f32) -> Self {
        fn box_compare(
            time0: f32,
            time1: f32,
            axis: usize,
        ) -> impl FnMut(&Rc<dyn Hittable>, &Rc<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if a_bbox.is_none() || b_bbox.is_none() {
                    panic!("No bounding box")
                }
                if a_bbox.unwrap().min[axis] - b_bbox.unwrap().min[axis] < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }

        let axis = rand::thread_rng().gen_range(0, 3) as usize;

        hittable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hittable.len();
        let (left, right) = if len == 1 {
            (hittable[0].clone(), hittable[0].clone())
        } else if len == 2 {
            (hittable[0].clone(), hittable[1].clone())
        } else {
            (
                Rc::new(BVHNode::new(&mut hittable[0..len / 2], time0, time1)) as Rc<dyn Hittable>,
                Rc::new(BVHNode::new(&mut hittable[len / 2..len], time0, time1))
                    as Rc<dyn Hittable>,
            )
        };

        let left_bbox = left.bounding_box(time0, time1);
        let right_bbox = right.bounding_box(time0, time1);
        if left_bbox.is_none() || right_bbox.is_none() {
            panic!("No bounding box")
        }

        BVHNode {
            left,
            right,
            bbox: surrounding_box(&left_bbox.unwrap(), &right_bbox.unwrap()),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let left = self.left.hit(&ray, t_min, t_max);
            let right = self.right.hit(&ray, t_min, t_max);
            match (left, right) {
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
