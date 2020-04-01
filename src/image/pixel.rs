#[derive(Debug, Clone, Copy)]

/// The structure of the RGB Pixel.
pub struct Pixel {
    /// The color red.
    pub r: u8,
    /// The color green.
    pub g: u8,
    /// The color blue.
    pub b: u8
}

impl Pixel {

    /// Create an RGB Pixel.
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {r: r, g: g, b: b}
    }
}

