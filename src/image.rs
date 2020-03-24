use std::io::Write;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {r: r, g: g, b: b}
    }
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8; 4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, b1]
}

pub struct Header {
    file_type: String,
    file_size: u32,
    res1: u8,
    res2: u8,
    pixel_data_offset: u8,
    header_size: u8,
    image_width: u32,
    image_height: u32,
    planes: u8,
    bits_per_pixel: u8,
    compression: u8,
    image_size: u32,
    x_per_meter: u8,
    y_per_meter: u8,
    color_map_entries_used: u8,
    sig_colors: u8
}

impl Header {
    pub fn new(
        image_width: u32, image_height: u32, image_size: u32) -> Header {
            Header {
                file_type: String::from("BM"), file_size: 0, res1: 0, res2: 0,
                pixel_data_offset: 54,  header_size: 40, image_width: image_width, 
                image_height: image_height, planes: 1, bits_per_pixel: 24, 
                compression: 0, image_size: image_size, x_per_meter: 0, 
                y_per_meter: 0,  color_map_entries_used: 0, sig_colors: 0        
            }    
        }

    pub fn get_binary_data(&self) -> Vec<u8>{
        let mut bdata: Vec<u8> = vec!();
        //File Type
        bdata.push(self.file_type.chars().nth(0).unwrap() as u8);
        bdata.push(self.file_type.chars().nth(1).unwrap() as u8);
        //File Size (is initally 0)
        for _i in 0..4 {
            bdata.push(self.file_size as u8);
        }
        //Res1  
        for _i in 0..2 {
            bdata.push(self.res1);  
        }
        //Res2  
        for _i in 0..2 {
            bdata.push(self.res2);  
        }
        //PixelDataOffset  
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.pixel_data_offset);
            }
            else {  
                bdata.push(0);  
            }
        }
        //Header Size 
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.header_size);
            }
            else {  
                bdata.push(0);  
            }
        }
        //Img Width
        let width = transform_u32_to_array_of_u8(self.image_width);
        for i in 0..4 {
            bdata.push(width[i]);
        }
        //Img Height
        let height = transform_u32_to_array_of_u8(self.image_height);
        for i in 0..4 {
            bdata.push(height[i]);
        }  
        //Planes 
        for i in 0..2 {
            if i == 0 {
                bdata.push(self.planes);
            }
            else {
                bdata.push(0);  
            }  
        }
        //BitsPerPixel
        for i in 0..2 {
            if i == 0 {
                bdata.push(self.bits_per_pixel);
            }
            else {
                bdata.push(0);  
            }  
        }
        //Compression
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.compression);
            }
            else {
                bdata.push(0);  
            }  
        }
        //ImageSize
        let image_size = transform_u32_to_array_of_u8(self.image_size);
        for i in 0..4 {
            bdata.push(image_size[i]);
        }
        //xPerMeter
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.x_per_meter);
            }
            else {
                bdata.push(0);  
            }  
        }
        //yPerMeter
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.y_per_meter);
            }
            else {
                bdata.push(0);  
            }  
        }
        //colorMapEntriesUsed
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.color_map_entries_used);
            }
            else {
                bdata.push(0);  
            }  
        }
        //sigColors
        for i in 0..4 {
            if i == 0 {
                bdata.push(self.sig_colors);
            }
            else {
                bdata.push(0);  
            }  
        }
        bdata
    }
}

pub struct Img {
    header: Header,
    pixels: Vec<Pixel>    
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
            data.push(pixel.b);
            data.push(pixel.g);
            data.push(pixel.r);
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
        panic!("I am not an image!");
    }
    let mut data = std::fs::File::create(path).unwrap();
    let img = Img::new(width, height, pixels);
    let bdata = img.get_binary_data();
    data.write(&bdata).unwrap();
}
