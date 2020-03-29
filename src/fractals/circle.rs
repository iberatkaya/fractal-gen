use crate::image::{Img, pixel::{Pixel}};

/**
 *  Implemented Bresenham’s circle drawing algorithm.
 *  For more details: https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
 */

pub(super) fn circle(radius: u32, image: &mut Img, color: Pixel){
    if radius > image.pixels.len() as u32 / 2 || radius > image.pixels[0].len() as u32 / 2 {
        panic!("Radius cannot be greater than half of width or heigth!")
    }
    bresenhams_circle(radius, &mut image.pixels, color);
}

pub(super) fn multiple_circles(radius: u32, frequency: u32, image: &mut Img, color: Pixel) {
    if radius > image.pixels.len() as u32 / 2 || radius > image.pixels[0].len() as u32 / 2 {
        panic!("Radius cannot be greater than half of width or heigth!")
    }
    let mut i = 1;
    while i < radius {
        bresenhams_circle(i, &mut image.pixels, color);
        i += frequency;
    }
}

fn bresenhams_circle(radius: u32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let xc = pixels.len() / 2;
    let yc = pixels[0].len() / 2;
    let mut x = 0;
    let mut y = radius as usize;
    let mut d: i32 = 3 - 2 * radius as i32;
    pixels[xc+x][yc+y as usize] = color;
    pixels[xc-x][yc+y as usize] = color;
    pixels[xc+x][yc-y as usize] = color;
    pixels[xc-x][yc-y as usize] = color;
    pixels[xc+y][yc+x as usize] = color;
    pixels[xc-y][yc+x as usize] = color;
    pixels[xc+y][yc-x as usize] = color;
    pixels[xc-y][yc-x as usize] = color;
    while y >= x {
        x += 1;
        if d > 0 {
            y -= 1;
            d += 4 * (x as i32 - y as i32) + 10;
        }
        else {
            d += 4 * x as i32 + 6;
        }
        pixels[xc+x][yc+y as usize] = color;
        pixels[xc-x][yc+y as usize] = color;
        pixels[xc+x][yc-y as usize] = color;
        pixels[xc-x][yc-y as usize] = color;
        pixels[xc+y][yc+x as usize] = color;
        pixels[xc-y][yc+x as usize] = color;
        pixels[xc+y][yc-x as usize] = color;
        pixels[xc-y][yc-x as usize] = color;
    }
}