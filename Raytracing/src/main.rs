mod camera;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;

extern crate num_cpus;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::camera::Camera;
use crate::dielectric::Dielectric;
use crate::hittable::{Hittable, HittableList};
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use hittable::HitRecord;
use image::ImageBuffer;
use lambertian::Lambertian;
use material::Material;
use nalgebra::Vector3;
use rand::Rng;

type Color = Vector3<f32>;

fn main_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::default();
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vector3::new(0.5, 0.5, 0.5)),
    ));
    for x in -11..11 {
        for y in -11..11 {
            let rand_material = rng.gen::<f32>();
            let center = Vector3::new(
                x as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                y as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - origin).magnitude() > 0.9 {
                if rand_material < 0.8 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vector3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    ));
                } else if rand_material < 0.95 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    ));
                } else {
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vector3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world
}

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

fn trace(
    lowerbound: i32,
    upperbound: i32,
    samples_per_pixel: i32,
    img_width: i32,
    img_height: i32,
    world: &HittableList,
    cam: &Camera,
) {
    let mut rng = rand::thread_rng();

    let num = num_cpus::get();
    let mut tasks = Vec::new();

    let rows_per_thread = (img_height as f32 / num as f32) as u32;

    let arc_world = Arc::new(world);
    let arc_image = Arc::new(Mutex::new(ImageBuffer::new(
        img_width as u32,
        img_height as u32,
    )));

    //Threads loop
    for cpu in 0..num {
        let thread_world = arc_world.clone();
        let thread_img = arc_image.clone();

        tasks.push(thread::spawn(move || {
            let top = if cpu == num - 1 || (cpu as u32 + 1) * rows_per_thread > img_height as u32 {
                img_height as u32
            } else {
                (cpu as u32 + 1) * rows_per_thread
            };

            let mut img_section = image::ImageBuffer::new(img_width as u32, img_height as u32);
        }));
    }

    //Single thread

    // for j in (lowerbound..upperbound).rev() {
    //     for i in 0..img_width {
    //         let mut pixel_color = Color::new(0.0, 0.0, 0.0);

    //         for _ in 0..samples_per_pixel {
    //             let u = (i as f32 + rng.gen::<f32>()) / img_width as f32;
    //             let v = (j as f32 + rng.gen::<f32>()) / img_height as f32;
    //             let ray = cam.get_ray(u, v);
    //             pixel_color += ray_color(&ray, &world, 0);
    //         }
    //         pixel_color /= samples_per_pixel as f32;
    //         for c in pixel_color.iter_mut() {
    //             *c = c.sqrt();
    //         }

    //         valueArray[i as usize][j as usize][0] = (255.99 * pixel_color[0]) as i32;
    //         valueArray[i as usize][j as usize][1] = (255.99 * pixel_color[1]) as i32;
    //         valueArray[i as usize][j as usize][2] = (255.99 * pixel_color[2]) as i32;

            // let ir = (255.99 * pixel_color[0]) as i32;
            // let ig = (255.99 * pixel_color[1]) as i32;
            // let ib = (255.99 * pixel_color[2]) as i32;
            // write_colorpixel(ir, ig, ib);
        }
    }
}

fn main() {
    //Init random

    //Image format
    let img_width: i32 = 400;
    let aspect_ratio = 16.0 / 9.0;
    let img_height: i32 = (img_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    //Monde raytraced
    let world = main_scene();
    let cam = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.0,
        10.0,
    );
    //Rendering colors
    print!("P3\n{0} {1}\n255\n", img_width, img_height);

    let chunkSize = img_height / 5;

    for i in 0..4 {
        threads.push(thread::spawn(move || {
            trace(
                chunkSize * i as i32,
                chunkSize * (i as i32 + 1),
                samples_per_pixel,
                img_width,
                img_height,
                &[world],
                &[cam],
            )
        }));
    }

    for i in threads.iter() {
        i.join();
    }

    //Todo: Multi-thread

    eprintln!("Processing done");
}

fn write_colorpixel(ir: i32, ig: i32, ib: i32) {
    print!("{0} {1} {2}\n", ir, ig, ib);
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
                return attenuation
                    .zip_map(&ray_color(&scattered, &world, depth + 1), |l, r| l * r);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let in_unit_sphere: Vector3<f32> = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    }

    -in_unit_sphere
}
