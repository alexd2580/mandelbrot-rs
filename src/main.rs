extern crate image;
use image::ImageBuffer;
extern crate num_complex;

use num_complex::{Complex, Complex64};

fn compute_pixel(x: i32, y: i32, w: i32, h: i32, iterations: u32) -> Complex64 {
    let normalized_x = x as f64 * (3.0 / w as f64) - 2.0;
    let normalized_y = y as f64 * (2.0 / h as f64) - 1.0;

    if iterations > 0 {
        let previous_z = compute_pixel(x, y, w, h, iterations - 1);
        &previous_z * &previous_z + Complex::new(normalized_x, normalized_y)
    } else {
        Complex::new(0.0, 0.0)
    }
}

fn init_image() -> () {
    ()
}

fn write_image() {}

fn main() {
    let mut img = ImageBuffer::new(512, 512);
    img.put_pixel(1, 1, image::Luma([255u8]));
    img.save("img.png");
    const W: i32 = 800;
    const H: i32 = 600;

    let iterations = 5;
    let image = init_image();

    for x in 0..W {
        for y in 0..H {
            let pixel = compute_pixel(x, y, W, H, iterations);
        }
    }

    write_image();
}
