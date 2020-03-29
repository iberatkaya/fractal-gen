use crate::image::{pixel::{Pixel}};

/**
 * Implemted DDA for drawing a line.
 * Checkout at: https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
 */

pub(super) fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let mut dx: i32;
    let mut dy: i32;
    let step: f64;
    let mut x: i32;
    let mut y: i32;

	dx = x2 - x1;
    dy = y2 - y1;
    if dx.abs() >= dy.abs() {
        step = dx.abs() as f64;
    }
    else {
        step = dy.abs() as f64;
    }
    if step == 0.0{
        return;
    }
    dx = (dx as f64 / step) as i32;
    dy = (dy as f64 / step) as i32;
    x = x1;
    y = y1;
    let mut i = 1;
    while i as f64 <= step {
        pixels[x as usize][y as usize] = color;
        x = x + dx;
        y = y + dy;
        i = i + 1;
    } 
}