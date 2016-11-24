extern crate image;
extern crate num_complex;
use image::ImageBuffer;

use num_complex::{Complex, Complex64};

fn compute_pixel(x: i32, y: i32, w: i32, h: i32, iterations: u32) -> Complex64 {
    Complex::new(0.0, 0.0)
}

fn init_image() -> () {
    ()
}

fn write_image() {}

fn main() {
    const w: u32 = 800;
    const h: u32 = 600;
    let iterations = 5;
    let d = generate_fractal(w, h, iterations);
    let image = init_image();
    //let mut img = ImageBuffer::new(w, h);
    
    //img.save("img.png");
}

fn generate_fractal(w: u32, h: u32, iterations: u32) -> Box<[u8]> {
    let mut vec_data = vec!(0u8; (w * h) as usize);
    let mut data = vec_data.into_boxed_slice();
    for x in 0..w {
        for y in 0..h {
            let pixel = compute_pixel(x as i32, y as i32, w as i32, h as i32, iterations);
            data[(y * w + x) as usize] = 0;
        }
    };
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
