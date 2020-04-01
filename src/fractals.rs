use std::io::Write;
use crate::image::{Img, pixel::Pixel};
mod circle;
use circle::{circle, multiple_circles};
mod line;
mod koch_curve;
use koch_curve::{koch_curve};
mod sierpinski_triangle;
use sierpinski_triangle::{triangle, trianglev2, sierpinski_triangle, multiple_triangles};
mod mandelbrot;
use mandelbrot::mandelbrot;


pub struct Fractal {
    pub image: Img
}


impl Fractal {
    pub fn new(pixels: Vec<Vec<Pixel>>) -> Fractal {
        let image = Img::new(pixels);
        Fractal {
            image: image
        }
    }
    pub fn write_image(&self, path: &str){
        if !path.contains(".bmp") {
            panic!("I am not a bmp image!");
        }
        let width = self.image.pixels[0].len();
        for row in self.image.pixels.iter(){
            if row.len() != width {
                panic!("The pixel array does not have equal row sizes!")
            }   
        }
        let mut data = std::fs::File::create(path).unwrap();
        let img = Img::new(self.image.pixels.clone());
        let bdata = img.get_binary_data();
        data.write(&bdata).unwrap();
    }
    pub fn circle(&mut self, xc: usize, yc: usize, radius: u32, color: Pixel){
        circle(xc, yc, radius, &mut self.image, color);
    }
    pub fn multiple_circles(&mut self, xc: usize, yc: usize, radius: u32, number: u32, color: Pixel){
        multiple_circles(xc, yc, radius, number, &mut self.image, color);
    }
    pub fn koch_curve(&mut self, p1x: i32, p1y: i32, p2x: i32, p2y: i32, amount: u32, color: Pixel){
        koch_curve(p1x, p1y, p2x, p2y, amount as i32, &mut self.image.pixels, color);
    }
    pub fn triangle(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        triangle(x, y, h, &mut self.image.pixels, color);
    }
    pub fn trianglev2(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        trianglev2(x, y, h, &mut self.image.pixels, color);
    }
    pub fn sierpinski_triangle(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        trianglev2(x, y, h * 2, &mut self.image.pixels, color);
        sierpinski_triangle(x as f64, y as f64, h as f64, &mut self.image.pixels, color);
    }
    pub fn multiple_triangles(&mut self, x: u32, y: u32, h: u32, number: u32, color: Pixel){
        multiple_triangles(x, y, h, number, &mut self.image.pixels, color);
    }
    pub fn mandelbrot(&mut self, color: Pixel){
        mandelbrot(&mut self.image.pixels, color);
    }
    pub fn rotate(&mut self){
        self.image.rotate();
    }
}