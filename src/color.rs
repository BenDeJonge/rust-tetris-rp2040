#![allow(dead_code)]

pub struct ColorRgb {
    /// A simple struct to model the RGB colorspace.
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    /// Convert an array of u8's to an RGB color.
    /// # Arguments
    /// - `array` - A reference to the array of u8's to convert
    /// # Returns
    /// - `ColorRgb` - An RGB color
    pub fn from_array(arr: &[u8; 3]) -> ColorRgb {
        ColorRgb {
            r: arr[0],
            g: arr[1],
            b: arr[2],
        }
    }

    /// Convert an RGB color to an array of u8's.
    /// # Returns
    /// - `[u8; 3]` - An array representation of the RGB colorspace
    pub fn to_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
