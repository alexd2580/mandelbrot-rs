extern crate image;
extern crate num_complex;
use image::ImageBuffer;

use num_complex::{Complex, Complex64};

fn compute_pixel(x: u32, y: u32, w: u32, h: u32, iterations: u32) -> Complex64 {
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
    const W: u32 = 800;
    const H: u32 = 600;

    let mut img = ImageBuffer::new(W, H);

    let iterations = 5;
    let d = generate_fractal(W, H, iterations);
    let image = init_image();
    // let mut img = ImageBuffer::new(w, h);

    // img.save("img.png");
}

fn generate_fractal(W: u32, H: u32, iterations: u32) -> Box<[u8]> {
    let mut vec_data = vec!(0u8; (W * H) as usize);
    let mut data = vec_data.into_boxed_slice();
    for x in 0..W {
        for y in 0..H {
            let pixel = compute_pixel(x, y, W, H, iterations);
            data[(y * W + x) as usize] = 0;
        }
    }
    data
}

mod tests {
    #[bench]
    fn benchmark_arc(b: &mut Bencher) {
        b.iter(|| generate_fractal_arc());
    }

    #[bench]
    fn benchmark_unsafe(b: &mut Bencher) {
        b.iter(|| generate_fractal_unsafe());
    }
}
