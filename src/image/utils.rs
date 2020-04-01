pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, b1]
}

pub fn round(num: f64) -> u64{
    let point = num - num as i64 as f64;
    if point >= 0.5 {
        return num as u64 + 1;
    }
    return num as u64;
}