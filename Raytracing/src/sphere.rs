use nalgebra::Vector3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Sphere{
    pub center: Vector3<f32>,
    pub radius: f32
}

impl Sphere{
    pub fn new(center: Vector3<f32>, radius: f32) -> Self{ Sphere {center, radius}}
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
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

        true
    }
}

