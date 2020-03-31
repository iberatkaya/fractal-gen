mod image;
use image::pixel::Pixel;
mod fractals;

fn main() {
    let mut pixels = vec![];    

    for _i in 0..1000 {
        let mut row = vec![];
        for _j in 0..1000 {
            row.push(Pixel::new(255, 255, 255));
        }
        pixels.push(row);
    }
    let mut image = fractals::Fractal::new(pixels);
    image.multiple_circles(200, 200, 200, 5, Pixel::new(250, 0, 0));
    image.write_image("./test.bmp");
}