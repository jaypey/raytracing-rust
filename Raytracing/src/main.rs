use std::string;

use nalgebra::Vector3;

type Color = Vector3<f64>;

fn main() {
    let img_height = 256;
    let img_width = 256;

    print!("P3\n{0} {1}\n255\n", img_width, img_height);
    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let pixel = Color::new(i as f64 / img_width as f64, j as f64 / img_height as f64, 0.20);
            write_colorpixel(pixel);
        }
    }
    eprintln!("Processing done");
}

fn write_colorpixel(c: Color) {
    print!("{0} {1} {2}\n", (255.999 * c[0]) as i32, (255.999 * c[1]) as i32, (255.999 * c[2]) as i32);
}
