use nalgebra::Vector3;
use crate::ray::Ray;

pub struct HitRecord{
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool
}
pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList{
    list: Vec<Box<dyn Hittable>>
}


impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord { p: Vector3::new(0.0,0.0,0.0), normal: Vector3::new(0.0,0.0,0.0), t: 0.0, front_face: false };
        let mut closest_so_far:f32 = t_max;
        let mut hit_anything = false;

        for object in self.list.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
            }
        }
        hit_anything

    }
}

impl HittableList{
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self{HittableList{list}}
}


impl HitRecord{
    pub fn set_face_normal(&mut self, r: &Ray, o_normal: Vector3<f32>){
        self.front_face = r.direction().dot(&o_normal) < 0.0;
        self.normal = if self.front_face { o_normal } else {-o_normal};

    }
}