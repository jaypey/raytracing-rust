use nalgebra::Vector3;
use crate::{material::Material, random_in_unit_sphere};
use crate::ray::Ray;


pub struct Lambertian{
    albedo: Vector3<f32>
}

impl Lambertian{
    pub fn new(albedo: Vector3<f32>) -> Self { Lambertian {albedo}}
}

impl Material for Lambertian{
    fn scatter(&self, ray: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> Option<(crate::ray::Ray, Vector3<f32>)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((scattered, self.albedo))
    }
}

