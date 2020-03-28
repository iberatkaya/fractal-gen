mod image;
use image::pixel::Pixel;
mod fractals;

fn main() {
    let mut pixels = vec![];
     for _i in 0..250 {
        let mut row = vec![];
        for _j in 0..250 {
            row.push(Pixel::new(255, 0, 0));
        }
        pixels.push(row);
    }
    let mut image = fractals::Fractal::new(pixels);
    image.draw_multiple_circles(120, 10);
    image.write_image("./test.bmp");
}
