use std::f32::consts::{FRAC_PI_2, PI};

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

fn get_sphere_uv(p: &Vector3<f32>) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + FRAC_PI_2) / PI;
    (u, v)
}

pub struct Sphere<M: Material> {
    pub center: Vector3<f32>,
    pub radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center_origin = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let half_b = center_origin.dot(&r.direction());
        let c = center_origin.dot(&center_origin) - self.radius.powi(2);

        let unknown = half_b * half_b - a * c;
        if unknown > 0.0 {
            let sqrt_discriminant = unknown.sqrt();
            let t = (-half_b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: &self.material,
                });
            }
            let t = (-half_b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: &self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        let radius = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }
}
