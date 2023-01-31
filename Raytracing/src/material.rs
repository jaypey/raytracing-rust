use crate::{ray::{Ray}, Color};
use nalgebra::Vector3;
use crate::hittable::HitRecord;

pub trait Material{
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

