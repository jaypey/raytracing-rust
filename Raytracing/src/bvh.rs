use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;

//Le bvh fonctionne d'une manière semblable à un arbre binaire.
enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut hittable: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn box_compare(
            time0: f32,
            time1: f32,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                //Comparaison des bounding boxes
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if a_bbox.is_none() || b_bbox.is_none() {
                    panic!("No bounding box")
                } else {
                    if a_bbox.unwrap().min[axis] - b_bbox.unwrap().min[axis] < 0.0 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        }

        fn axis_range(
            hittable: &Vec<Box<dyn Hittable>>,
            time0: f32,
            time1: f32,
            axis: usize,
        ) -> f32 {
            let (min, max) = hittable
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    if let Some(aabb) = hit.bounding_box(time0, time1) {
                        (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|a| (a, axis_range(&hittable, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        hittable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hittable.len();
        match len {
            0 => panic!("no elements"),
            1 => {
                let leaf = hittable.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!["no bounding box"]
                }
            }
            _ => {
                let right = BVH::new(hittable.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(hittable, time0, time1);
                let bbox = surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(&ray, t_min, t_max),
                BVHNode::Branch { left, right } => {
                    let left = left.hit(&ray, t_min, t_max);
                    if let Some(l) = &left {
                        t_max = l.t
                    };
                    let right = right.hit(&ray, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
