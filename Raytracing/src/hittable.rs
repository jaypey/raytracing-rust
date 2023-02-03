use nalgebra::Vector3;
use crate::lambertian::Lambertian;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a>{
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: &'a dyn Material
}
pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f32, t_max:f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList{
    list: Vec<Box<dyn Hittable>>
}

unsafe impl Send for HittableList {}

unsafe impl Sync for HittableList {}


impl HittableList{
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self{HittableList{list}}

    pub fn push(&mut self, hitable: impl Hittable + 'static){
        self.list.push(Box::new(hitable))
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max:f32) -> Option<HitRecord> {
        let mut closest_so_far:f32 = t_max;
        let mut hit_anything = None;

        for object in self.list.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything

    }
}