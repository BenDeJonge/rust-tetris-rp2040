//! A module containing a `Tetromino` struct, modelling a tetromino piece.

#![allow(dead_code)]

use crate::color::{Name, Rgb};
use crate::coordinate::Coordinate;
use crate::rotation::generate_matrices;
use array2d::Array2D;

// TODO: how to save a list of TetrominoShapes, each with color and array. Generate e.g. Vec<Tetromino>

/// An enum of available tetrominoshapes
pub enum TetrominoShape {
    /// x x x x
    I,
    /// x . .
    /// x x x
    J,
    /// . . x
    /// x x x
    L,
    /// x x
    /// x x
    O,
    /// . x x
    /// x x .
    S,
    /// . x .
    /// x x x
    T,
    /// x x .
    /// . x x
    Z,
}

/// A struct reflecting a Tetromino block.
pub struct Tetromino<T> {
    /// - `shape` - A public `TetrominoShape` enum variant representing the shape
    pub shape: TetrominoShape,
    /// - `color` - A public `ColorRgb` struct representing the LED color
    pub color: Rgb,
    /// - `masks` - An array of binary masks for the 4 rotation states
    masks: [Array2D<T>; 4],
    /// - `index` - The index of the currently used mask
    index: usize,
}

impl<T> Tetromino<T>
where
    T: Clone,
{
    /// Create a new `Tetromino` based on a shape.
    /// # Arguments
    /// - `shape` - A `Tetrominoshape` enum variant representing the shape
    /// - `color` - A `ColorRgb` struct representing the red, green and blue component
    /// - `mask` - An initial mask as an `Array2D<T>`, to be rotated three times
    /// # Returns
    /// - `Tetromino` - An instance of a Tetromino struct
    pub fn new(shape: TetrominoShape, color: Rgb, mask: Array2D<T>) -> Self {
        Tetromino {
            shape,
            color,
            masks: generate_matrices(mask),
            index: 0,
        }
    }

    /// Get the current mask.
    /// # Returns
    /// - `&Array2D<bool>` - A reference to currently valid binary mask
    pub fn get_mask(&self) -> &Array2D<T> {
        &self.masks[self.index]
    }

    /// Get the shape of the current mask.
    /// # Returns
    /// - `[usize; 2]` - The shape of the current mask as number of rows and number of columns.
    pub fn get_shape(&self) -> Coordinate {
        Coordinate::from_array([self.get_mask().num_rows(), self.get_mask().num_columns()])
    }

    /// Get the bottom right coordinate of the current board state.
    /// # Returns
    /// - `Coordinate` - The bottom right coordinate, equal to [row - 1, col - 1]
    pub fn get_coords(&self) -> Coordinate {
        self.get_shape() - [1, 1]
    }

    /// Increment the index, representing a rotation of 90 degrees clockwise.
    pub fn rotate_cw(&mut self) {
        self.index = (self.index + 1) % self.masks.len();
    }

    /// Decrement the index, representing a rotation of 90 degrees clockwise.
    pub fn rotate_ccw(&mut self) {
        self.index = (self.index + self.masks.len() - 1) % self.masks.len();
    }
}

impl From<TetrominoShape> for Tetromino<bool> {
    /// Convert from a `TetrominoShape` to a `Tetromino`.
    fn from(shape: TetrominoShape) -> Self {
        match shape {
            s @ TetrominoShape::I => Tetromino::new(
                s,
                Rgb::from(Name::Cyan),
                Array2D::from_row_major(
                    &[
                        true, true, true, true, // o o o o
                    ],
                    1,
                    4,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::J => Tetromino::new(
                s,
                Rgb::from(Name::Blue),
                Array2D::from_row_major(
                    &[
                        true, false, false, //  o . .
                        true, true, true, //    o o o
                    ],
                    2,
                    3,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::L => Tetromino::new(
                s,
                Rgb::from(Name::Orange),
                Array2D::from_row_major(
                    &[
                        false, false, true, //  . . o
                        true, true, true, //    o o o
                    ],
                    2,
                    3,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::O => Tetromino::new(
                s,
                Rgb::from(Name::Yellow),
                Array2D::from_row_major(
                    &[
                        true, true, // o o
                        true, true, // o o
                    ],
                    2,
                    2,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::S => Tetromino::new(
                s,
                Rgb::from(Name::Green),
                Array2D::from_row_major(
                    &[
                        false, true, true, // . x x
                        true, true, false, // x x .
                    ],
                    2,
                    3,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::T => Tetromino::new(
                s,
                Rgb::from(Name::Purple),
                Array2D::from_row_major(
                    &[
                        false, true, false, //  . x .
                        true, true, true, //    x x x
                    ],
                    2,
                    3,
                )
                .unwrap(),
            ),

            s @ TetrominoShape::Z => Tetromino::new(
                s,
                Rgb::from(Name::Purple),
                Array2D::from_row_major(
                    &[
                        true, true, false, //   x x .
                        false, true, true, //   . x x
                    ],
                    2,
                    3,
                )
                .unwrap(),
            ),
        }
    }
}

#[cfg(test)]

mod tests {
    use crate::rotation::{rotate_ccw, rotate_cw};

    use super::{Tetromino, TetrominoShape};
    use array2d::Array2D;

    #[test]
    fn test_tetromino_init() {
        // Create S Tetromino, check if array matches.
        let t_s = Tetromino::from(TetrominoShape::S);
        let m_s = Array2D::from_row_major(
            &[
                false, true, true, // . x x
                true, true, false, // x x .
            ],
            2,
            3,
        )
        .unwrap();
        assert_eq!(t_s.get_mask(), &m_s);
    }

    #[test]
    fn test_tetromino_rotate_cw() {
        // Create J Tetromino, rotate 10 times clockwise, check if all match.
        let mut t_j = Tetromino::from(TetrominoShape::J);
        let mut m_j = Array2D::from_row_major(
            &[
                true, false, false, // o . .
                true, true, true, //   o o o
            ],
            2,
            3,
        )
        .unwrap();
        for _ in 0..10 {
            t_j.rotate_cw();
            m_j = rotate_cw(&m_j);
            assert_eq!(t_j.get_mask(), &m_j);
        }
    }

    #[test]
    fn test_tetromino_rotate_ccw() {
        // Create Z Tetromino, rotate 10 times counterclockwise, check if all match.
        let mut t_z = Tetromino::from(TetrominoShape::Z);
        let mut m_z = Array2D::from_row_major(
            &[
                true, true, false, // x x .
                false, true, true, // . x x
            ],
            2,
            3,
        )
        .unwrap();
        for _ in 0..10 {
            t_z.rotate_ccw();
            m_z = rotate_ccw(&m_z);
            assert_eq!(t_z.get_mask(), &m_z);
        }
    }
}
