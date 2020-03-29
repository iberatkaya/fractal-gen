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
    image.snowflake(50, 50, 950, 950, 5, Pixel::new(0, 0, 255));
    image.draw_multiple_circles(450, 100, Pixel::new(0, 155, 0));
    image.write_image("./test.bmp");
}