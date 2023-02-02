use nalgebra::{Vector3, Vector};
use rand::Rng;

use crate::{material::Material, random_in_unit_sphere, ray::Ray};

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>,  ni_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0{
        let refracted = ni_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    }else{
        None
    }
}

pub struct Dielectric{
    ir: f32
}

impl Dielectric{
    pub fn new(ir: f32) -> Self{
        Dielectric {ir: ir}
    }
}

impl Material for Dielectric{
    fn scatter(&self, ray: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> Option<(crate::ray::Ray, Vector3<f32>)> {
        let attenuation = Vector3::new(1.0,1.0,1.0);
        let (o_normal, ni_nt, cos) = if ray.direction().dot(&hit.normal) > 0.0{
            let cos = self.ir * ray.direction().dot((&hit.normal)) / ray.direction().magnitude();
            (-hit.normal, self.ir, cos)
        }else{
            let cos = -ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (hit.normal, 1.0/self.ir, cos)
        };

        if let Some(refracted) = refract(&ray.direction(), &o_normal, ni_nt){

            // Ajout de la probabilité de réflection avec schlick
            let r0 = ((1.0 - self.ir) / (1.0 + self.ir)).powi(2);
            let reflect_prob = r0 + (1.0 - r0) * (1.0 - cos).powi(2);
            if rand::thread_rng().gen::<f32>() >= reflect_prob {
                return Some((Ray::new(hit.p, refracted), attenuation));
            }
        }

        let reflected = reflect(&ray.direction(), &hit.normal);
        Some((Ray::new(hit.p, reflected), attenuation))
    }
}