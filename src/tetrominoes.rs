#![allow(dead_code)]

use crate::color::{Color, ColorRgb};
use crate::coordinate::Coordinate;
use crate::rotation::generate_matrices;
use array2d::Array2D;

// TODO: how to save a list of TetrominoShapes, each with color and array. Generate e.g. Vec<Tetromino>

pub enum TetrominoShape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Tetromino<T> {
    /// A struct reflecting a Tetromino block.
    /// # Attributes
    /// - `shape` - A public `TetrominoShape` enum variant representing the shape
    /// - `color` - A public `ColorRgb` struct representing the LED color
    /// - `masks` - An array of binary masks for the 4 rotation states
    /// - `index` - The index of the currently used mask
    pub shape: TetrominoShape,
    pub color: ColorRgb,
    masks: [Array2D<T>; 4],
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
    pub fn new(shape: TetrominoShape, color: ColorRgb, mask: Array2D<T>) -> Self {
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
            TetrominoShape::I => Tetromino {
                shape: TetrominoShape::I,
                color: ColorRgb::from(Color::Cyan),
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            true, true, true, true, // o o o o
                        ],
                        1,
                        4,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::J => Tetromino {
                shape: TetrominoShape::J,
                color: ColorRgb::from(Color::Blue),
                index: 0,
                masks: generate_matrices(
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
            },

            TetrominoShape::L => Tetromino {
                shape: TetrominoShape::L,
                color: ColorRgb::from(Color::Orange),
                index: 0,
                masks: generate_matrices(
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
            },

            TetrominoShape::O => Tetromino {
                shape: TetrominoShape::O,
                color: ColorRgb::from(Color::Yellow),
                index: 0,
                masks: generate_matrices(
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
            },

            TetrominoShape::S => Tetromino {
                shape: TetrominoShape::S,
                color: ColorRgb::from(Color::Green),
                index: 0,
                masks: generate_matrices(
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
            },

            TetrominoShape::T => Tetromino {
                shape: TetrominoShape::T,
                color: ColorRgb::from(Color::Purple),
                index: 0,
                masks: generate_matrices(
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
            },

            TetrominoShape::Z => Tetromino {
                shape: TetrominoShape::Z,
                color: ColorRgb::from(Color::Red),
                index: 0,
                masks: generate_matrices(
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
            },
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
