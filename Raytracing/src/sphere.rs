use nalgebra::Vector3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<M: Material>{
    pub center: Vector3<f32>,
    pub radius: f32,
    material: M
}

impl<M: Material> Sphere<M>{
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self{ Sphere {center, radius, material}}
}

impl<M: Material> Hittable for Sphere<M>{
    fn hit(& self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let center_origin = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let half_b = center_origin.dot(&r.direction());
        let c = center_origin.dot(&center_origin) - self.radius.powi(2);

        let unknown = half_b*half_b-a*c;
        if unknown > 0.0 {
            let sqrt_discriminant = unknown.sqrt();
            let t = (-half_b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal, material: &self.material })
            }
            let t = (-half_b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal, material: &self.material })
            }
        }
        None
    }
}

