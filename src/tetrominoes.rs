#![allow(dead_code)]

use crate::color::ColorRgb;
use crate::rotation::generate_matrices;
use array2d::Array2D;

pub enum TetrominoShape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
pub struct Tetromino {
    /// A struct reflecting a Tetromino block.
    /// # Attributes
    /// - `shape` - A public `TetrominoShape` enum variant representing the shape
    /// - `color` - A public `ColorRgb` struct representing the LED color
    /// - `masks` - An array of binary masks for the 4 rotation states
    /// - `index` - The index of the currently used mask
    pub shape: TetrominoShape,
    pub color: ColorRgb,
    masks: [Array2D<bool>; 4],
    index: usize,
}

impl Tetromino {
    /// Create a new `Tetromino` based on a shape.
    /// # Arguments
    /// - `shape` - A `Tetrominoshape` enum variant representing the shape
    /// # Returns
    /// - `Tetromino` - An instance of a Tetromino struct
    pub fn new(shape: TetrominoShape) -> Self {
        match shape {
            TetrominoShape::I => Tetromino {
                shape: TetrominoShape::I,
                color: ColorRgb::from_array(&[0, 255, 255]), // cyan
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, false, // . . . .
                            false, false, false, false, // . . . .
                            true, true, true, true, //     o o o o
                            false, false, false, false, // . . . .
                        ],
                        4,
                        4,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::J => Tetromino {
                shape: TetrominoShape::J,
                color: ColorRgb::from_array(&[0, 0, 255]), // blue
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, // . . .
                            true, false, false, //  o . .
                            true, true, true, //    o o o
                        ],
                        3,
                        3,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::L => Tetromino {
                shape: TetrominoShape::L,
                color: ColorRgb::from_array(&[255, 127, 0]), // orange
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, // . . .
                            false, false, true, //  . . o
                            true, true, true, //    o o o
                        ],
                        3,
                        3,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::O => Tetromino {
                shape: TetrominoShape::O,
                color: ColorRgb::from_array(&[255, 255, 0]), // yellow
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
                color: ColorRgb::from_array(&[0, 255, 0]), // green
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, // . . .
                            false, true, true, //   . x x
                            true, true, false, //   x x .
                        ],
                        3,
                        3,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::T => Tetromino {
                shape: TetrominoShape::T,
                color: ColorRgb::from_array(&[255, 0, 255]), // purple
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, // . . .
                            false, true, false, //  . x .
                            true, true, true, //    x x x
                        ],
                        3,
                        3,
                    )
                    .unwrap(),
                ),
            },

            TetrominoShape::Z => Tetromino {
                shape: TetrominoShape::Z,
                color: ColorRgb::from_array(&[255, 0, 0]), // red
                index: 0,
                masks: generate_matrices(
                    Array2D::from_row_major(
                        &[
                            false, false, false, // . . .
                            true, true, false, //   x x .
                            false, true, true, //   . x x
                        ],
                        3,
                        3,
                    )
                    .unwrap(),
                ),
            },
        }
    }

    /// Get the current mask.
    /// # Returns
    /// - `&Array2D<bool>` - A reference to currently valid binary mask
    pub fn get_mask(&self) -> &Array2D<bool> {
        &self.masks[self.index]
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

#[cfg(test)]

mod tests {
    use crate::rotation::{rotate_ccw, rotate_cw};

    use super::{Tetromino, TetrominoShape};
    use array2d::Array2D;

    #[test]
    fn test_tetromino_init() {
        // Create S Tetromino, check if array matches.
        let t_s = Tetromino::new(TetrominoShape::S);
        let m_s = Array2D::from_row_major(
            &[
                false, false, false, // . . .
                false, true, true, //   . x x
                true, true, false, //   x x .
            ],
            3,
            3,
        )
        .unwrap();
        assert_eq!(t_s.get_mask(), &m_s);
    }

    #[test]
    fn test_tetromino_rotate_cw() {
        // Create J Tetromino, rotate 10 times clockwise, check if all match.
        let mut t_j = Tetromino::new(TetrominoShape::J);
        let mut m_j = Array2D::from_row_major(
            &[
                false, false, false, // . . .
                true, false, false, //  o . .
                true, true, true, //    o o o
            ],
            3,
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
        let mut t_z = Tetromino::new(TetrominoShape::Z);
        let mut m_z = Array2D::from_row_major(
            &[
                false, false, false, // . . .
                true, true, false, //   x x .
                false, true, true, //   . x x
            ],
            3,
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
