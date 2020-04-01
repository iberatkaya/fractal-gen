use crate::image::{pixel::{Pixel}};

/**
 * Implemted DDA for drawing a line.
 * Checkout at: https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
 */

pub(super) fn draw_line(x1: u32, y1: u32, x2: u32, y2: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let dx: f64;
    let dy: f64;
    let step: f64;
    let mut x: f64;
    let mut y: f64;

	dx = x2 as f64 - x1 as f64;
    dy = y2 as f64 - y1 as f64;
    if dx.abs() >= dy.abs() {
        step = dx.abs() as f64;
    }
    else {
        step = dy.abs() as f64;
    }
    if step == 0.0{
        return;
    }
    let x_inc = dx as f64 / step;
    let y_inc = dy as f64 / step;
    x = x1 as f64;
    y = y1 as f64;
    let mut i = 0.0;
    while i <= step {
        pixels[x as usize][y as usize] = color;
        x = x + x_inc;
        y = y + y_inc;
        i = i + 1.0;
    } 
}