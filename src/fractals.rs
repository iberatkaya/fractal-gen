use std::io::Write;
use crate::image::{Img, pixel::{Pixel}};
mod circle;
use circle::{circle, multiple_circles};

pub struct Fractal {
    image: Img
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
    pub fn draw_circle(&mut self, radius: u32){
        self.image = circle(radius, self.image.clone());
    }
    pub fn draw_multiple_circles(&mut self, radius: u32, frequency: u32){
        self.image = multiple_circles(radius, frequency, self.image.clone());
    }
}