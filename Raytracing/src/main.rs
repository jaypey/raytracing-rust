mod aabb;
mod bvh;
mod camera;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod texture;

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
use image::{GenericImageView, ImageBuffer, Rgb};
use lambertian::Lambertian;
use material::Material;
use nalgebra::Vector3;
use rand::Rng;
use std::time::Instant;
use texture::{CheckerTexture, SolidColor};

type Color = Vector3<f32>;

fn main_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::default(); //Le world est en fait simplement une liste d'objets qui peuvent être atteint par des rayons.

    //Dans cette liste, je push une sphere en indiquant son rayon, la position de son centre et le matériau
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(CheckerTexture::new(
            SolidColor::new(0.0, 0.4, 0.1),
            SolidColor::new(1.0, 1.0, 01.0),
        )),
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
                        Lambertian::new(SolidColor::new(
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
        Lambertian::new(SolidColor::new(4.0, 2.0, 0.0)),
    ));
    world.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector3::new(0.7, 0.8, 0.6), 0.1),
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

// Fonction principale qui effectue le raytracing
fn trace(
    samples_per_pixel: i32,
    img_width: i32,
    img_height: i32,
    world: HittableList,
    cam: Camera,
) {
    let num = num_cpus::get() - 2;
    let mut tasks = Vec::new();

    let rows_per_thread = (img_height as f32 / num as f32) as u32;

    let arc_world = Arc::new(world);
    let arc_image = Arc::new(Mutex::new(image::RgbImage::new(
        img_width as u32,
        img_height as u32,
    )));

    //Threads loop
    for cpu in 0..num {
        let thread_world = arc_world.clone();
        let thread_img = arc_image.clone();

        tasks.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let top = if cpu == num - 1 || (cpu as u32 + 1) * rows_per_thread > img_height as u32 {
                img_height as u32
            } else {
                (cpu as u32 + 1) * rows_per_thread
            };

            let mut img_section = image::RgbImage::new(img_width as u32, img_height as u32);

            for j_inverse in (cpu as u32 * rows_per_thread)..top {
                let j = img_height as u32 - j_inverse - 1;
                for i in 0..img_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    // Samples per pixel, est le nombre de rayon qui sera envoyé pour un pixel
                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + rng.gen::<f32>()) / img_width as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / img_height as f32;
                        //Va chercher la direction et la position du vecteur qui représente le rayon.
                        let ray = cam.get_ray(u, v);
                        pixel_color += ray_color(&ray, &*thread_world, 0);
                    }
                    //Diviser la couleur par le nombre de pixels pour obtenir une couleur moyenne.
                    pixel_color /= samples_per_pixel as f32;
                    for c in pixel_color.iter_mut() {
                        *c = c.sqrt();
                    }

                    //Ajouter le pixel à l'image
                    img_section.put_pixel(
                        i as u32,
                        j_inverse - (cpu as u32 * rows_per_thread),
                        image::Rgb {
                            0: [
                                (255.99 * pixel_color[0]) as u8,
                                (255.99 * pixel_color[1]) as u8,
                                (255.99 * pixel_color[2]) as u8,
                            ],
                        },
                    )
                }
            }

            let mut img_data = thread_img.lock().unwrap();

            println!("End of thread: {0}", cpu);

            for i in 0..img_width {
                for j_inverse in (cpu as u32 * rows_per_thread)..top {
                    img_data.put_pixel(
                        i as u32,
                        j_inverse,
                        *img_section
                            .get_pixel(i as u32, j_inverse - (cpu as u32 * rows_per_thread)),
                    );
                }
            }
        }));
    }

    for task in tasks {
        let _ = task.join();
    }

    let output_path = "output.png";

    image::DynamicImage::ImageRgb8(arc_image.lock().unwrap().clone())
        .save(output_path)
        .unwrap();
}

fn main() {
    let startTime = Instant::now();

    //Image format
    let img_width: i32 = 1920;
    let aspect_ratio = 16.0 / 9.0;
    let img_height: i32 = (img_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 60;

    //Monde raytraced
    let world = main_scene();
    let cam = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0,
    );
    //Rendering colors
    eprintln!("Starting rendering....");

    trace(samples_per_pixel, img_width, img_height, world, cam);

    let elapsed = startTime.elapsed();
    eprintln!("Processing done in : {:.2?}", elapsed);
}

//Retourne une couleur.
fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
                //Fait rebondir les rayons de manière récursif en prenant en compte le matériau touché.
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
