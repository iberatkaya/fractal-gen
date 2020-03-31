use crate::image::{pixel::{Pixel}};
use super::line::draw_line;

pub fn triangle(x: u32, y: u32, h: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel) { 
    draw_line((x as f64 - (h as f64 / 3.0_f64.sqrt())) as u32, (y - h / 3) as u32, (x as f64 + (h as f64 / 3.0_f64.sqrt())) as u32, (y - h / 3) as u32, pixels, color); 
    draw_line((x as f64 - (h as f64 / 3.0_f64.sqrt())) as u32, (y - h / 3) as u32, x as u32, (y + 2 * h / 3) as u32, pixels, color); 
    draw_line(x as u32, (y + 2 * h / 3) as u32, (x as f64 + (h as f64 / 3.0_f64.sqrt())) as u32, (y - h / 3) as u32, pixels, color); 
} 

pub fn multiple_triangles(x: u32, y: u32, h: u32, number: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let mut i = h;
    let div = h / number;
    while i > 0 {
        draw_line((x as f64 - (i as f64 / 3.0_f64.sqrt())) as u32, (y - i / 3) as u32, (x as f64 + (i as f64 / 3.0_f64.sqrt())) as u32, (y - i / 3) as u32, pixels, color); 
        draw_line((x as f64 - (i as f64 / 3.0_f64.sqrt())) as u32, (y - i / 3) as u32, x as u32, (y + 2 * i / 3) as u32, pixels, color); 
        draw_line(x as u32, (y + 2 * i / 3) as u32, (x as f64 + (i as f64 / 3.0_f64.sqrt())) as u32, (y - i / 3) as u32, pixels, color); 
        i -= div;
    }
}

pub fn trianglev2(x: u32, y: u32, h: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel) { 
    draw_line((x as f64 - (h as f64 / 3.0_f64.sqrt())) as u32, (y + h / 3) as u32, (x as f64 + (h as f64 / 3.0_f64.sqrt())) as u32, (y + h / 3) as u32, pixels, color); 
    draw_line((x as f64 - (h as f64 / 3.0_f64.sqrt())) as u32, (y + h / 3) as u32, x as u32, (y - 2 * h / 3) as u32, pixels, color); 
    draw_line(x as u32, (y - 2 * h / 3) as u32, (x as f64 + (h as f64 / 3.0_f64.sqrt())) as u32, (y + h / 3) as u32, pixels, color); 
} 

pub fn sierpinski_triangle(x: f64, y: f64, h: f64, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    if h < 5.0 {
        return;
    }
    if x > 0.0 && y > 0.0 {
        triangle(x as u32, y as u32, h as u32, pixels, color);
    }
    sierpinski_triangle(x, y - 2.0 * h / 3.0, h / 2.0, pixels, color);
    sierpinski_triangle(x as f64 - h as f64 / 3_f64.sqrt(), y + h / 3.0, h / 2.0, pixels, color);
    sierpinski_triangle(x as f64 + h as f64 / 3_f64.sqrt(), y + h / 3.0, h / 2.0, pixels, color);

}
