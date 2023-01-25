use std::string;

mod ray;

use nalgebra::Vector3;
use crate::ray::Ray;

type Color = Vector3<f32>;

fn main() {

    //Image format
    let img_width = 400;
    let aspect_ratio = 16.0/9.0;
    let img_height = (img_width as f32 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    //Position de la camera
    let origin = Vector3::new(0.0,0.0,0.0);

    //Position du max horizontal
    let horizontal = Vector3::new(viewport_width,0.0,0.0);
    //Position du max vertical
    let vertical = Vector3::new(0.0,viewport_height,0.0);
    //Position du coin bas gauche du plan de projection.
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0,0.0,focal_length);

    //Rendering colors
    print!("P3\n{0} {1}\n255\n", img_width, img_height);
    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let u = i as f32 / (img_width-1) as f32;
            let v = j as f32 / (img_height-1) as f32;
            let ray = Ray::new(origin, lower_left_corner+u*horizontal+v*vertical-origin);
            let pixel = ray_color(&ray);
            write_colorpixel(pixel);
        }
    }
    eprintln!("Processing done");
}

fn write_colorpixel(c: Color) {
    print!("{0} {1} {2}\n", (255.999 * c[0]) as i32, (255.999 * c[1]) as i32, (255.999 * c[2]) as i32);
}

fn ray_color(r: &Ray) -> Color{
    let vec_unit:Vector3<f32> = r.direction().normalize();
    let t = 0.5*(vec_unit[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}