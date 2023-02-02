mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dielectric;

use hittable::HitRecord;
use lambertian::Lambertian;
use material::Material;
use nalgebra::{Vector3};
use rand::Rng;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::dielectric::Dielectric;

type Color = Vector3<f32>;

fn main_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -1000.0,0.0), 1000.0, Lambertian::new(Vector3::new(0.5,0.5,0.5))));
    for x in -11..11{
        for y in -11..11 {
            let rand_material = rng.gen::<f32>();
            let center = Vector3::new(x as f32 + 0.9 * rng.gen::<f32>(), 0.2, y as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if rand_material < 0.8{
                    world.push(Sphere::new(center, 0.2, Lambertian::new(Vector3::new(rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>()))));
                } else if rand_material < 0.95 {
                    world.push(Sphere::new(center, 0.2, Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>())));
                }
                else{
                    world.push( Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector3::new(0.4, 0.2, 0.1))));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));
    world
}


fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0,1.0,1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        if p.magnitude_squared() < 1.0{
            return p;
        }
    }
}
fn main() {

    //Init random
    let mut rng = rand::thread_rng();



    //Image format
    let img_width = 400;
    let aspect_ratio = 16.0/9.0;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;


    //Monde raytraced
    let world = main_scene();
    let cam = Camera::new(Vector3::new(13.0,2.0,3.0), Vector3::new(0.0,0.0, 0.0), Vector3::new(0.0,1.0,0.0),20.0, 16.0/9.0, 0.0, 10.0);
    //Rendering colors
    print!("P3\n{0} {1}\n255\n", img_width, img_height);

    //Todo: Multi-thread
    for j in (0..img_height).rev() {
        eprint! ("\x1B[2J\x1B[1;1H");
        eprintln!("{0}/{1} Processing", j, img_height);
        for i in 0..img_width {
            let mut pixel_color = Color::new(0.0,0.0,0.0);

            for _ in 0..samples_per_pixel{
                    let u = (i as f32 + rng.gen::<f32>()) / img_width as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / img_height as f32;
                    let ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, 0);
            }
            pixel_color /= samples_per_pixel as f32;
            for c in pixel_color.iter_mut() { *c = c.sqrt(); }
            let ir = (255.99 * pixel_color[0]) as i32;
            let ig = (255.99 * pixel_color[1]) as i32;
            let ib = (255.99 * pixel_color[2]) as i32;
            write_colorpixel(ir, ig, ib);
        }
    }
    eprintln!("Processing done");
}


fn write_colorpixel(ir: i32, ig: i32, ib: i32) {
    print!("{0} {1} {2}\n", ir, ig, ib);
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color{
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
                return attenuation.zip_map(&ray_color(&scattered, &world, depth+1), |l, r| l * r);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32>{
    let in_unit_sphere:Vector3<f32> = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    }

    -in_unit_sphere
}