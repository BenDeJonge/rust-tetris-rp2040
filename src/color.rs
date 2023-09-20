//! A module modelling the available game colors in a `Rgb` struct.

#![allow(dead_code)]

/// A simple struct to model the RGB colorspace.
pub struct Rgb {
    /// The red component
    pub r: u8,
    /// The green component
    pub g: u8,
    /// The blue component
    pub b: u8,
}

impl Rgb {
    /// Convert an array of u8's to an RGB color.
    /// # Arguments
    /// - `array` - A reference to the array of u8's to convert
    /// # Returns
    /// - `ColorRgb` - An RGB color
    pub fn from_array(arr: [u8; 3]) -> Rgb {
        Rgb {
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

#[derive(Default)]
/// An enum of available RGB colors in the game
pub enum Name {
    /// The color blue
    Blue,
    /// The color cyan
    Cyan,
    #[default]
    /// The color green, which is default
    Green,
    /// The color orange
    Orange,
    /// The color purple
    Purple,
    /// The color red
    Red,
    /// The color yellow
    Yellow,
}

impl From<Name> for Rgb {
    fn from(color: Name) -> Self {
        match color {
            Name::Blue => Rgb::from_array([0, 0, 255]),
            Name::Cyan => Rgb::from_array([0, 255, 255]),
            Name::Green => Rgb::from_array([0, 255, 0]),
            Name::Orange => Rgb::from_array([255, 127, 0]),
            Name::Purple => Rgb::from_array([255, 0, 255]),
            Name::Red => Rgb::from_array([255, 0, 0]),
            Name::Yellow => Rgb::from_array([255, 255, 0]),
        }
    }
}
