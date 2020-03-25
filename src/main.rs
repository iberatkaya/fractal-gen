mod image;

fn main() {
    let mut pixels = vec![];
    for _i in 0..30 {
        for j in 0..=255 {
            pixels.push(image::pixel::Pixel::new(j, 0, 0));
        }
    }
    for _i in 0..30 {
        for j in 0..=255 {
            pixels.push(image::pixel::Pixel::new(0, j, 0));
        }
    }
    for _i in 0..30 {
        for j in 0..=255 {
            pixels.push(image::pixel::Pixel::new(0, 0, j));
        }
    }
    image::write_image("./test.bmp", 256, 90, pixels);
}
