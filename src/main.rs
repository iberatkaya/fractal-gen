mod image;

fn main() {
    let mut pixels = vec![];
    for _i in 0..30 {
        let mut row = vec![];
        for j in 0..=255 {
            row.push(image::pixel::Pixel::new(j, j, 0));
        }
        pixels.push(row);
    }
    for _i in 0..30 {
        let mut row = vec![];
        for j in 0..=255 {
            row.push(image::pixel::Pixel::new(j, j, j));
        }
        pixels.push(row);
    }
    for _i in 0..30 {
        let mut row = vec![];
        for j in 0..=255 {
            row.push(image::pixel::Pixel::new(0, 0, j));
        }
        pixels.push(row);
    }
    image::write_image("./test.bmp", pixels);
}
