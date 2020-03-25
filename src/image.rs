use std::io::Write;
pub mod pixel;
pub mod header;
use header::Header;
use pixel::Pixel;
use std::convert::TryInto;
mod utils;
use utils::transform_u32_to_array_of_u8;

pub struct Img {
    header: Header,
    pub pixels: Vec<Pixel>    
}

impl Img {
    pub fn new(width: u32, height: u32, pixels: Vec<Pixel>) -> Img {
        let header = Header::new(width, height, width * height);
        Img {
            header: header, pixels: pixels
        }
    }
    pub fn get_binary_data(&self) -> Vec<u8>{
        let mut data = self.header.get_binary_data();
        for pixel in self.pixels.iter() {
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
        }
        for _i in 0..(4 - (self.pixels.len() % 4)){
            data.push(0);
            data.push(0);
            data.push(0);
        }
        let size = transform_u32_to_array_of_u8(data.len() as u32);
        for i in 0..4 {
            data[i+2] = size[i];
        }
        data
    }
}


pub fn write_image(path: &str, width: u32, height: u32, pixels: Vec<Pixel>){
    if !path.contains(".bmp") {
        panic!("I am not a bmp image!");
    }
    if width * height != pixels.len().try_into().unwrap() {
        panic!("The pixel array does not have the same size as the image!");
    }
    let mut data = std::fs::File::create(path).unwrap();
    let img = Img::new(width, height, pixels);
    let bdata = img.get_binary_data();
    data.write(&bdata).unwrap();
}
