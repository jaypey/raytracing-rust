use nalgebra::{Vector3, Vector};
use crate::ray::Ray;
use rand::Rng;

fn random_in_unit_disk() -> Vector3<f32> {
    let mut random = rand::thread_rng();
    let unit = Vector3::new(1.0,1.0,0.0);
    loop {
        let p = 2.0 * Vector3::new(random.gen::<f32>(), random.gen::<f32>(), 0.0) - unit;
        if p.dot(&p) < 1.0{
            return p
        }
    }
}

pub struct Camera{
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lens_radius: f32,
    u: Vector3<f32>,
    v: Vector3<f32>,
}

impl Camera{
    pub fn new(lookfrom: Vector3<f32>, lookat: Vector3<f32>, vup: Vector3<f32>, vfov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Self{


        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = focus_distance * f32::tan(theta/2.0);
        let half_width = aspect_ratio * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Camera { 
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w * focus_distance,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray{
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}