extern crate image;
use image::ImageBuffer;

fn main() {
    let mut img = ImageBuffer::new(512, 512);
    img.put_pixel(1, 1, image::Luma([255u8]));
    img.save("img.png");
}
