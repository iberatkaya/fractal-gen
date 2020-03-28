use crate::image::{Img, pixel::{Pixel}};

/**
 *  Implemented Bresenham’s circle drawing algorithm.
 *  For more details: https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
 */

pub(super) fn circle(radius: u32, mut image: Img) -> Img{
    if radius > image.pixels.len() as u32 / 2 || radius > image.pixels[0].len() as u32 / 2 {
        panic!("Radius cannot be greater than half of width or heigth!")
    }
    image.pixels = bresenhams_circle(radius, image.pixels);
    image
}

pub(super) fn multiple_circles(radius: u32, frequency: u32, mut image: Img) -> Img {
    if radius > image.pixels.len() as u32 / 2 || radius > image.pixels[0].len() as u32 / 2 {
        panic!("Radius cannot be greater than half of width or heigth!")
    }
    let mut i = 1;
    while i < radius {
        image.pixels = bresenhams_circle(i, image.pixels);
        i += frequency;
    }
    image
}

fn bresenhams_circle(radius: u32, mut pixels: Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>>{
    let xc = pixels.len() / 2;
    let yc = pixels[0].len() / 2;
    let mut x = 0;
    let mut y = radius as usize;
    let mut d: i32 = 3 - 2 * radius as i32;
    pixels[xc+x][yc+y as usize] = Pixel::new(0, 0, 0);
    pixels[xc-x][yc+y as usize] = Pixel::new(0, 0, 0);
    pixels[xc+x][yc-y as usize] = Pixel::new(0, 0, 0);
    pixels[xc-x][yc-y as usize] = Pixel::new(0, 0, 0);
    pixels[xc+y][yc+x as usize] = Pixel::new(0, 0, 0);
    pixels[xc-y][yc+x as usize] = Pixel::new(0, 0, 0);
    pixels[xc+y][yc-x as usize] = Pixel::new(0, 0, 0);
    pixels[xc-y][yc-x as usize] = Pixel::new(0, 0, 0);
    while y >= x {
        x += 1;
        if d > 0 {
            y -= 1;
            d += 4 * (x as i32 - y as i32) + 10;
        }
        else {
            d += 4 * x as i32 + 6;
        }
        pixels[xc+x][yc+y as usize] = Pixel::new(0, 0, 0);
        pixels[xc-x][yc+y as usize] = Pixel::new(0, 0, 0);
        pixels[xc+x][yc-y as usize] = Pixel::new(0, 0, 0);
        pixels[xc-x][yc-y as usize] = Pixel::new(0, 0, 0);
        pixels[xc+y][yc+x as usize] = Pixel::new(0, 0, 0);
        pixels[xc-y][yc+x as usize] = Pixel::new(0, 0, 0);
        pixels[xc+y][yc-x as usize] = Pixel::new(0, 0, 0);
        pixels[xc-y][yc-x as usize] = Pixel::new(0, 0, 0);
    }
    pixels
}