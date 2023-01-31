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
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: & mut HitRecord<'a>) -> bool{
        let center_origin = r.origin() - self.center;
        let a = r.direction().magnitude_squared();
        let half_b = center_origin.dot(&r.direction());
        let c = center_origin.magnitude_squared() - self.radius * self.radius;

        let unknown = half_b*half_b-a*c;
        if unknown < 0.0{
            return false;  
        }
        let sqrt = f32::sqrt(unknown);

        let mut root = (-half_b - sqrt)/a;
        if root < t_min || t_max < root{
            root = (-half_b + sqrt)/a;
            if root < t_min || t_max < root{
                return false;
            }
        }

        rec.t = root;
        rec.p = r.point_at(rec.t);
        let o_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, o_normal);
        rec.material = &self.material;
        true
    }
}

