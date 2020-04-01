//! Fractal Gen is a Rust project to generate fractals.
//! 
//! # Examples
//! 
//! ``` 
//! let mut pixels = vec![];    
//! 
//! for _i in 0..1000 {
//!     let mut row = vec![];
//!     for _j in 0..1000 {
//!         row.push(Pixel::new(255, 255, 255));
//!     }
//!     pixels.push(row);
//! }
//! let mut image = Fractal::new(pixels);
//! image.multiple_circles(825, 825, 125, 6, Pixel::new(0, 150, 150));
//! image.write_image("./test.bmp");
//! ```
//! 

pub mod fractal;
pub mod image;