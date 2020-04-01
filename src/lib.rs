//! Fractal Gen is a Rust project to generate fractals and save them as BMP images.
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

#![deny(missing_docs)]
/// Generate fractals and save them as BMP images.
pub mod fractal;
/// Generate images and save them as BMP files.
pub mod image;