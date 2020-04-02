use std::io::Write;
use crate::image::{Img, pixel::Pixel};
mod circle;
use circle::{circle, multiple_circles};
mod line;
mod koch_curve;
use koch_curve::{koch_curve};
mod sierpinski_triangle;
use sierpinski_triangle::{triangle, trianglev2, sierpinski_triangle, multiple_triangles};
mod mandelbrot;
use mandelbrot::mandelbrot;
mod tree;
use tree::tree;
mod barnsley_fern;
use barnsley_fern::barnsley_fern; 
mod julia;
use julia::{julia, julia_colored};


/// The structure of the Fractal.
pub struct Fractal {
    /// The fractal's image data.
    pub image: Img
}


impl Fractal {


    /// Create a Fractal by giving the pixels as a 2D Vector.
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut image = Fractal::new(pixels);
    /// ```
    pub fn new(pixels: Vec<Vec<Pixel>>) -> Fractal {
        let image = Img::new(pixels);
        Fractal {
            image: image
        }
    }



    /// Write the image as a bmp file in the given path.
    /// 
    /// # Example
    /// 
    /// ```
    /// let image = image::Img::new(pixels);
    /// image.write_image("./myimage.bmp");
    /// ```
    pub fn write_image(&self, path: &str){
        if !path.contains(".bmp") {
            panic!("I am not a bmp image!");
        }
        let width = self.image.pixels[0].len();
        for row in self.image.pixels.iter(){
            if row.len() != width {
                panic!("The pixel array does not have equal row sizes!")
            }   
        }
        let mut data = std::fs::File::create(path).unwrap();
        let img = Img::new(self.image.pixels.clone());
        let bdata = img.get_binary_data();
        data.write(&bdata).unwrap();
    }


    /// Generate a circle by giving the center coordinates, the radius, and the color.
    pub fn circle(&mut self, xc: usize, yc: usize, radius: u32, color: Pixel){
        circle(xc, yc, radius, &mut self.image, color);
    }
    

    /// Generate repeating circle patterns where each circle's radius is decremented by number.
    ///
    /// # Example
    ///
    /// ```
    /// image.multiple_circles(825, 825, 125, 6, Pixel::new(0, 150, 150));
    /// ```
    pub fn multiple_circles(&mut self, xc: usize, yc: usize, radius: u32, number: u32, color: Pixel){
        multiple_circles(xc, yc, radius, number, &mut self.image, color);
    }


    /// Generate a Koch Curve by giving the start and end coordinates, the color, and the amount of recursion.
    /// Keep in mind that the time to run increases significantly as amount increases.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.koch_curve(675, 75, 925, 325, 5, Pixel::new(0, 250, 0));
    /// ```
    pub fn koch_curve(&mut self, p1x: i32, p1y: i32, p2x: i32, p2y: i32, amount: u32, color: Pixel){
        koch_curve(p1x, p1y, p2x, p2y, amount as i32, &mut self.image.pixels, color);
    }


    /// Generate a triange by giving its coordinates, height and color.
    pub fn triangle(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        triangle(x, y, h, &mut self.image.pixels, color);
    }


    /// Generate a triange by giving its coordinates, height and color.
    pub fn trianglev2(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        trianglev2(x, y, h, &mut self.image.pixels, color);
    }


    /// Generate a Sierpinksi Triange by giving its coordinates, height, and color.
    pub fn sierpinski_triangle(&mut self, x: u32, y: u32, h: u32, color: Pixel){
        trianglev2(x, y, h * 2, &mut self.image.pixels, color);
        sierpinski_triangle(x as f64, y as f64, h as f64, &mut self.image.pixels, color);
    }


    /// Generate repeating triangles where each triangles height is decremented by number.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.multiple_triangles(175, 800, 220, 6, Pixel::new(250, 0, 250));
    /// ```
    pub fn multiple_triangles(&mut self, x: u32, y: u32, h: u32, number: u32, color: Pixel){
        multiple_triangles(x, y, h, number, &mut self.image.pixels, color);
    }

    
    /// Generate a Mandelbrot Set with the given color.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.mandelbrot(Pixel::new(250, 0, 0));
    /// ```
    pub fn mandelbrot(&mut self, color: Pixel){
        mandelbrot(&mut self.image.pixels, color);
    }


    /// Rotate the fractal by 90 degrees.
    pub fn rotate(&mut self){
        self.image.rotate();
    }


    /// Generate a tree by giving its coordinates, height, angle, growth, and color.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.tree(100, 800, 100, PI / 4.0, 2, Pixel::new(0, 0, 255));
    /// ```
    pub fn tree(&mut self, x: u32, y: u32, h: u32, angle: f64, growth: u32, color: Pixel){
        tree(x, y, h, angle, growth, &mut self.image.pixels, color);
    }

    /// Generate a fern by selecting its coordinates, number of iterations, and color.
    /// The more iterations, the better the shape becomes a fern. A good value is about 5000000.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.barnsley_fern(500, 100, 5000000, Pixel::new(255, 0, 0));
    /// ```
    pub fn barnsley_fern(&mut self, x: i32, y: u32, iterations: u32, color: Pixel){
        barnsley_fern(x as i32 - self.image.pixels.len() as i32, y, iterations, &mut self.image.pixels, color);
    }


    /// Generate a Julia Set.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.julia();
    /// ```
    pub fn julia(&mut self){
        julia(&mut self.image.pixels);
    }


    /// Generate a Julia Set with a mixture of colors given in the pixel parameter.
    /// 
    /// # Example
    /// 
    /// ```
    /// image.julia_colored(Pixel::new(250, 150, 100));
    /// ```
    pub fn julia_colored(&mut self, color: Pixel){
        julia_colored(&mut self.image.pixels, color);
    }
}