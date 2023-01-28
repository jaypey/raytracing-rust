mod ray;
mod sphere;
mod hittable;
mod camera;

use hittable::HitRecord;
use nalgebra::{Vector3};
use rand::Rng;
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::camera::Camera;

type Color = Vector3<f32>;


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
    let max_depth = 50;


    //Monde raytraced
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0,0.0,-1.0),0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0),100.0))
    ]);
    let cam = Camera::new();
    //Rendering colors
    print!("P3\n{0} {1}\n255\n", img_width, img_height);

    //Todo: Multi-thread
    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..samples_per_pixel{
                
                    let u = (i as f32 + rng.gen::<f32>()) / (img_width-1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (img_height-1) as f32;
                    let ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, max_depth);
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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color{

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec= HitRecord { p: Vector3::new(0.0,0.0,0.0), normal: Vector3::new(0.0,0.0,0.0), t: 0.0, front_face: false };
    if(world.hit(r, 0.001, f32::MAX, &mut rec)){
        let target = rec.p + random_in_hemisphere(&rec.normal);
        return 0.5*ray_color(&Ray::new(rec.p, target - rec.p), world, depth-1);
    }

    let u_direction = r.direction().normalize();
    let t = 0.5 * (u_direction[1] + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * t

    
}

fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32>{
    let in_unit_sphere:Vector3<f32> = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    }

    -in_unit_sphere
}