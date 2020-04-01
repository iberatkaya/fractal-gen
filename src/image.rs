use std::io::Write;
pub mod pixel;
pub mod header;
use header::Header;
use pixel::Pixel;
pub mod utils;
use utils::*;
mod rotate;
use rotate::rotate;

pub struct Img {
    header: Header,
    pub pixels: Vec<Vec<Pixel>>    
}

impl Img {

    
    ///Create a BMP image.
    pub fn new(pixels: Vec<Vec<Pixel>>) -> Img {
        let height = pixels.len() as u32;
        let width = pixels[0].len() as u32;
        let header = Header::new(width, height, width * height);
        Img {
            header: header, pixels: pixels
        }
    }


    ///Get the image's binary data.
    pub fn get_binary_data(&self) -> Vec<u8>{
        let mut data = self.header.get_binary_data();
        for i in (0..self.pixels.len()).rev(){
            for pixel in self.pixels[i].iter() {
                data.push(pixel.b);
                data.push(pixel.g);
                data.push(pixel.r);
            }
            for _i in 0..(4 - (self.pixels[0].len() as u32 * 3) % 4){
                if (self.pixels[0].len() as u32 * 3) % 4 == 0 {
                    break;
                }
                data.push(0);
            }
        }
        let mut size = transform_u32_to_array_of_u8(data.len() as u32);
        for i in 0..4 {
            data[i+2] = size[i];
        }
        size = transform_u32_to_array_of_u8(self.pixels.len() as u32 * ((self.pixels[0].len() as u32 * 3 + 7) & !7) as u32 / 16);
        for i in 0..4 {
            data[i+34] = size[i];
        }
        data
    }


    ///Write the image into a file.
    pub fn write_image(&self, path: &str){
        if !path.contains(".bmp") {
            panic!("I am not a bmp image!");
        }
        let width = self.pixels[0].len();
        for row in self.pixels.iter(){
            if row.len() != width {
                panic!("The pixel array does not have equal row sizes!")
            }   
        }
        let mut data = std::fs::File::create(path).unwrap();
        let img = Img::new(self.pixels.clone());
        let bdata = img.get_binary_data();
        data.write(&bdata).unwrap();
    }


    ///Rotate the image by 90 degrees.
    pub fn rotate(&mut self){
        self.pixels = rotate(&mut self.pixels);
    }
}