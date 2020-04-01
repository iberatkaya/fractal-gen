use super::pixel::Pixel;

pub fn rotate(pixels: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>>{
    let mut newarr = pixels.clone();
    let mut i: i32 = pixels[0].len() as i32 - 1;
    let mut j: i32 = pixels.len() as i32 - 1;
    while i >= 0 {
        while j >= 0 {
            newarr[i as usize][j as usize] = pixels[j as usize][pixels[0].len() - i as usize - 1];
            j -= 1;
        }
        j = pixels.len() as i32 - 1;
        i -= 1;
    }
    newarr
}