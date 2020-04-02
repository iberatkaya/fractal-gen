# Fractal Gen

[![Crate](https://img.shields.io/crates/v/fractal-gen.svg)](https://crates.io/crates/fractal-gen) [![API](https://docs.rs/fractal-gen/badge.svg)](https://docs.rs/fractal-gen)

> Fractal Gen is a Rust project to generate fractals.

Fractals is a [Crate](https://crates.io/crates/fractal-gen) that generate fractals as images without any external dependencies. The images are saved as a BMP files. Checkout the [log](https://github.com/iberatkaya/fractals/blob/master/LOG.md) and [documentation](https://docs.rs/fractal-gen/).

### Some Fractals:

* Mandelbrot Set
* Julia Set
* Koch Curve
* Sierpinski Triangle
* Barnsley Fern
* Tree



<p align="center">
    <img alt="Logo" src="https://raw.githubusercontent.com/iberatkaya/fractals/master/examples/compressed/fractals.jpg" width="400" height="400">
</p>

```rust
extern crate fractal_gen;
use fractal_gen::{image::pixel::Pixel, fractal::Fractal};


fn main() {
    // The pixel array
    let mut pixels = vec![];    

    // Height = 1000px
    for _i in 0..1000 {
        let mut row = vec![];
        // Width = 1000px
        for _j in 0..1000 {
            row.push(Pixel::new(255, 255, 255));
        }
        pixels.push(row);
    }
    // Create a Fractal from the pixels.
    let mut image = Fractal::new(pixels);

    // Draw the Mandelbrot Set on the image.
    image.mandelbrot(Pixel::new(250, 0, 0));

    // Draw a Sierpinksi Triangle on the image.
    image.sierpinski_triangle(180, 180, 100, Pixel::new(0, 0, 250));
    
    // Draw a Koch Curve on the image.
    image.koch_curve(675, 75, 925, 325, 5, Pixel::new(0, 250, 0));

    // Write the BMP image.
    image.write_image("./test.bmp");
}

```

## Author

👤 **Ibrahim Berat Kaya**

* Github: [@iberatkaya](https://github.com/iberatkaya)
* LinkedIn: [@linkedin.com/in/ibrahim-berat-kaya/](https://linkedin.com/in/ibrahim-berat-kaya/)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check the [issues page](https://github.com/iberatkaya/fractals/issues). 
