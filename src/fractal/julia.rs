use crate::image::{pixel::{Pixel}};

/**
 * Implemented The Julia Set.
 * Checkout at: https://rosettacode.org/wiki/Julia_set
 **/

pub fn julia(pixels: &mut Vec<Vec<Pixel>>){
        let cx = -0.9;
        let cy = 0.27015;
        let iterations = 110;
     
        for x in 0..pixels[0].len() {
            for y in 0..pixels.len() {
     
                let mut zx = 3.0 * (x as f64 - 0.5 * pixels[0].len() as f64) / (pixels[0].len() as f64);
                let mut zy = 2.0 * (y as f64 - 0.5 * pixels.len() as f64) / (pixels.len() as f64);
     
                let mut i = iterations;
     
                while zx * zx + zy * zy < 4.0 && i > 1 {
                    let tmp = zx * zx - zy * zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = tmp;
                    i -= 1;
                }     
                let pixel = Pixel::new(i << 3, i << 5, i << 4);
                pixels[x as usize][y as usize] = pixel;
            }
        }
}

pub fn julia_colored(pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let cx = -0.9;
    let cy = 0.27015;
    let iterations = 110;
 
    for x in 0..pixels[0].len() {
        for y in 0..pixels.len() {
 
            let mut zx = 3.0 * (x as f64 - 0.5 * pixels[0].len() as f64) / (pixels[0].len() as f64);
            let mut zy = 2.0 * (y as f64 - 0.5 * pixels.len() as f64) / (pixels.len() as f64);
 
            let mut i = iterations;
 
            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                i -= 1;
            }     
            let pixel = Pixel::new(((color.r as u32 * i) % 255) as u8, ((color.g as u32 * i) % 255) as u8, ((color.b as u32 * i) % 255) as u8);
            pixels[x as usize][y as usize] = pixel;
        }
    }
}