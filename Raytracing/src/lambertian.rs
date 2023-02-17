use crate::ray::Ray;
use crate::texture::Texture;
use crate::{material::Material, random_in_unit_sphere};
use nalgebra::Vector3;

pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit: &crate::hittable::HitRecord,
    ) -> Option<(crate::ray::Ray, Vector3<f32>)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }
}
