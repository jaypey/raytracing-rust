use nalgebra::Vector3;

use crate::{material::Material, random_in_unit_sphere, ray::Ray};

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

pub struct Metal{
    albedo: Vector3<f32>,
    fuzz: f32
}

impl Metal{
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self{
        Metal { albedo, fuzz:if fuzz < 1.0 {fuzz} else {1.0}}
    }
}

impl Material for Metal{
    fn scatter(&self, ray: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> Option<(crate::ray::Ray, Vector3<f32>)> {
        let mut reflected = reflect(&ray.direction().normalize(), &hit.normal);
        if self.fuzz > 0.0 {reflected += self.fuzz * random_in_unit_sphere()};
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected);
            Some((scattered, self.albedo))
        }else{
            None
        }
    }
}