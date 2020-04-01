# Fractal Gen

> Fractal Gen is a Rust project to generate fractals.

Fractals is a project designed to generate fractals as images without any external dependencies. Checkout the [log](https://github.com/iberatkaya/fractals/blob/master/LOG.md). Currently under development. 

<p align="center">
    <img alt="Repeating Circles" src="https://raw.githubusercontent.com/iberatkaya/fractals/master/examples/compressed/fractals.jpg" width="400" height="400">
</p>

```rust
extern crate fractal_gen;
use fractal_gen::{image::pixel::Pixel, fractal::Fractal};


fn main() {
    let mut pixels = vec![];    

    for _i in 0..1000 {
        let mut row = vec![];
        for _j in 0..1000 {
            row.push(Pixel::new(255, 255, 255));
        }
        pixels.push(row);
    }
    let mut image = Fractal::new(pixels);
    image.mandelbrot(Pixel::new(250, 0, 0));
    image.sierpinski_triangle(180, 180, 100, Pixel::new(0, 0, 250));
    image.koch_curve(675, 75, 925, 325, 5, Pixel::new(0, 250, 0));
    image.multiple_triangles(175, 800, 220, 6, Pixel::new(250, 0, 250));
    image.multiple_circles(825, 825, 125, 6, Pixel::new(0, 150, 150));
    image.write_image("./test.bmp");
}

```

## Author

👤 **Ibrahim Berat Kaya**

* Github: [@iberatkaya](https://github.com/iberatkaya)
* LinkedIn: [@linkedin.com/in/ibrahim-berat-kaya/](https://linkedin.com/in/ibrahim-berat-kaya/)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check the [issues page](https://github.com/iberatkaya/fractals/issues). 
