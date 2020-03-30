use crate::image::{pixel::{Pixel}};
use super::line::draw_line;
use std::f64::consts::PI;

/**
 * Implemented Koch's curve.
 * Checkout at: http://computergraphicsinc.blogspot.com/2014/12/c-program-to-draw-koch-curve-using.html
 */

pub fn koch_curve(p1x: i32, p1y: i32, p2x: i32, p2y: i32, amount: i32, pixels: &mut Vec<Vec<Pixel>>, color: Pixel){
    let px: i32;
    let py: i32;
    let p3x: i32;
    let p3y: i32;
    let p4x: i32;
    let p4y: i32;
    let theta: f64 = PI / 3.0;

    p3x = (2*p1x+p2x)/3;
    p3y = (2*p1y+p2y)/3;
    p4x = (2*p2x+p1x)/3;
    p4y = (2*p2y+p1y)/3;
    px = (p3x as f64  + (p4x - p3x) as f64 * theta.cos() + (p4y - p3y) as f64 * theta.sin()) as i32;
    py = (p3y as f64  - (p4x - p3x) as f64 * theta.sin() + (p4y - p3y) as f64 * theta.cos()) as i32;

	if amount > 0 {
 

		koch_curve(p1x, p1y, p3x, p3y, amount-1, pixels, color);
		koch_curve(p3x, p3y, px, py, amount-1, pixels, color);
		koch_curve(px, py, p4x, p4y, amount-1, pixels, color);
		koch_curve(p4x, p4y, p2x, p2y, amount-1, pixels, color);
	}
	else{
		draw_line(p1x as u32, p1y as u32, p3x as u32, p3y as u32, pixels, color);
		draw_line(p3x as u32, p3y as u32, px as u32, py as u32, pixels, color);
		draw_line(px as u32, py as u32, p4x as u32, p4y as u32, pixels, color);
		draw_line(p4x as u32, p4y as u32, p2x as u32, p2y as u32, pixels, color);
    }
}