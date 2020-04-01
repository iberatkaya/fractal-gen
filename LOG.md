# Log

## Below is a log of my experience with developing this project. 

### 01.04.2020:

* Added a tree fractal inspired from [Nature of Code](https://natureofcode.com/book/chapter-8-fractals/).

### 31.03.2020:

* Implemented The Mandelbrot Set. Some useful links: [The Mandelbrot Set](http://warp.povusers.org/Mandelbrot/), [Mandelbrot Set in C](https://rosettacode.org/wiki/Mandelbrot_set#C).
* Implemented a image rotating function.

### 30.03.2020: 

* Implemented [Sierpinski's Triangle](https://www.geeksforgeeks.org/sierpinski-triangle-using-graphics/). 

### 29.03.2020:

* Implemented [DDA](https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)) for drawing a line. 
* Implemented Koch Curve for generating snowflake patterns. Example codes in C: [Rosetta](https://rosettacode.org/wiki/Koch_curve#C), [computergraphicsinc](http://computergraphicsinc.blogspot.com/2014/12/c-program-to-draw-koch-curve-using.html).

### 28.03.2020:

* Implemented [Bresenham's circle drawing algorithm](https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/).
* Drew a circle and periodic circles on a bmp image.

### 26.03.2020:

* Fixed several bugs while creating a bmp image.
* This was by far the most detailed and helpful tutorial to create a bmp image: [BMP File Format](http://www.di.unito.it/~marcog/SM/BMPformat-Wiki.pdf).
* While debugging, I learnt to read a bmp image by looking at the bytes, which is an interesting skill.

### 24.03.2020:

* Learnt about structs and methods in Rust.
* Created structs for the project.
* Successfully generated an bmp image using the structs with width 255 and height 90.
* Used a function from the Rust Forum implemented by [nateozem](https://users.rust-lang.org/t/how-to-serialize-a-u32-into-byte-array/986/5) to convert a u64 to a u8 array.

### 23.03.2020:

* Learnt about bmp files. Sample links: [Bitmap images in C](http://ricardolovelace.com/creating-bitmap-images-with-c-on-windows.html), [The bmp file format](http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm), [BMP](http://www.onicos.com/staff/iz/formats/bmp.html).
* Developed a template for generating bmp files. Generated a red pixel and 2x1 red and green pixel bmp image.