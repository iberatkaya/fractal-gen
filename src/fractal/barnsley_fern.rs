use crate::image::{pixel::{Pixel}};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn barnsley_fern(x: i32, y: u32, iterations: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let mut x0: f64 = 0.0;
    let mut y0: f64 = 0.0;
    let mut x1: f64;
    let mut y1: f64;
    let width = pixels[0].len();
    for j in 0..iterations {
        // Use as a random number
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let num = (time) % 101;
        if num == 0 {
            x1 = 0.0;
            y1 = 0.16 * y0;
        }
        else if num >= 1 && num <= 7 {
            x1 = -0.15 * x0 + 0.28 * y0;
            y1 = 0.26 * x0 + 0.24 * y0 + 0.44;
        }
        else if num >= 8 && num <= 15 {
            x1  = 0.2 * x0 - 0.26 * y0;
            y1 = 0.13 * x0 + 0.22 * y0 + 1.6;
        }
        else {
            x1 = 0.85 * x0 + 0.04 * y0;
            y1 = -0.04 * x0 + 0.85 * y0 + 1.6;
        }
        pixels[(x as f64 + 30.0 * x1 + width as f64) as usize][(y as f64 + 30.0 * y1) as usize] = color;
        x0 = x1;
        y0 = y1;
    }
}