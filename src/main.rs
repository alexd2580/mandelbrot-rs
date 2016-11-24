extern crate image;
use image::ImageBuffer;
extern crate num_complex;

use num_complex::{Complex, Complex64};

fn compute_pixel(x: i32, y: i32, w: i32, h: i32, iterations: u32) -> Complex64 {
    
    Complex::new(0.0, 0.0)
}

fn init_image() -> () {
    ()
}

fn write_image() {}

fn main() {
    let mut img = ImageBuffer::new(512, 512);
    img.put_pixel(1, 1, image::Luma([255u8]));
    img.save("img.png");
    const w: i32 = 800;
    const h: i32 = 600;

    let iterations = 5;
    let image = init_image();

    for x in 0..w {
        for y in 0..h {
            let pixel = compute_pixel(x, y, w, h, iterations);
        }
    }

    write_image();
}
