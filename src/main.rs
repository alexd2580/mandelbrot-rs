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

fn main() {
    const W: u32 = 800;
    const H: u32 = 600;
    let iterations = 5;
    let d = generate_fractal(W, H, iterations);
    let mut img = ImageBuffer::from_fn(W, H, |x, y| {
        image::Luma([d[(y * W + x) as usize]])
    });

    img.save("img.png");
}

fn generate_fractal(W: u32, H: u32, iterations: u32) -> Box<[u8]> {
    let mut vec_data = vec!(0u8; (W * H) as usize);
    let mut data = vec_data.into_boxed_slice();
    for x in 0..W {
        for y in 0..H {
            let comp = compute_pixel(x, y, W, H, iterations);
            data[(y * W + x) as usize] = (comp.norm().ln() * 255.0) as u8;
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
