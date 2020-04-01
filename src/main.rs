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
    image.mandelbrot(Pixel::new(250, 0, 0));
    image.sierpinski_triangle(180, 180, 100, Pixel::new(0, 0, 250));
    image.koch_curve(675, 75, 925, 325, 5, Pixel::new(0, 250, 0));
    image.multiple_triangles(175, 800, 220, 6, Pixel::new(250, 0, 250));
    image.multiple_circles(825, 825, 125, 6, Pixel::new(0, 150, 150));
    image.write_image("./test.bmp");
}