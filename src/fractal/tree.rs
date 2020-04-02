use crate::image::{pixel::{Pixel}, utils::round};
use super::line::draw_line;

/**
 * Implemented a tree fractal.
 * Checkout at: https://natureofcode.com/book/chapter-8-fractals/ 
 **/

pub fn tree(x: u32, y: u32, h: u32, angle: f64, growth: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    draw_line(x, y, x + h, y, pixels, color);
    branch(x, y, h, angle, growth, pixels, color);
}

pub fn branch(x: u32, y: u32, h: u32, angle: f64, growth: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    if growth <= 1 {
        panic!("Growth must be greater than 1!");
    }
    let x1 = x;
    let y1 = y;
    let x2 = round(x as f64 - (h / growth) as f64 * angle.cos()) as u32;
    let y2 = round(y as f64 - (h / growth) as f64 * angle.sin()) as u32 - 1;
    let y2i = round(y as f64 + (h / growth) as f64 * angle.sin()) as u32;
    if ((x1 - x2) as f64).abs() < 2.0 {
        return;
    } 
    draw_line(x1, y1, x2, y2, pixels, color);
    draw_line(x1, y1, x2, y2i, pixels, color);
    branch(x2, y2, h / growth, angle, growth, pixels, color);   
    branch(x2, y2i, h / growth, angle, growth, pixels, color);
}