use crate::image::{pixel::{Pixel}};

/**
 * Implemented The Mandelbrot Set.
 * Checkout at: http://warp.povusers.org/Mandelbrot/
 */

pub fn mandelbrot(pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let min_re: f64 = -2.5;
    let max_re: f64 = 1.5;
    let min_im: f64 = -2.0;
    let max_im: f64 = 2.0;
    let re_factor: f64 = (max_re - min_re) / (pixels[0].len() as f64);
    let im_factor: f64 = (max_im - min_im) / (pixels.len() as f64);
    let max_it: u32 = 200;
    for y in 0..(pixels.len()) {
        let c_im: f64 = max_im - y as f64 * im_factor;
        for x in 0..(pixels[0].len()) {
            let c_re: f64 = min_re + x as f64 * re_factor;
            let mut z_re: f64 = c_re;
            let mut z_im: f64 = c_im;
            let mut is_inside = true;
            let mut ctr: u32 = 0;
            for n in 0..max_it {
                ctr = n;
                let z_re2: f64 = z_re * z_re;
                let z_im2: f64 = z_im * z_im;
                if z_im2 + z_re2 > 4.0 {
                    is_inside = false;
                    break;
                }
                z_im = 2.0 * z_re * z_im + c_im;
                z_re = z_re2 - z_im2 + c_re;
            }
            if ctr == max_it-1 {
                pixels[x][y] = Pixel::new(0, 0, 0);
            }
            if is_inside && x < pixels.len() && y < pixels[0].len() {
                pixels[x][y] = color;
            }
        }
    }
}