//! A module containing a `Board<T>` struct, modelling the current board state of the game

#![allow(dead_code)]

use crate::coordinate::Coordinate;
use array2d::{Array2D, Error};
use std::cmp::{max, min};

/// A struct modelling a current board of placed tetrominos
pub struct Board<T: Copy> {
    /// The current board state
    board: Array2D<T>,
    /// The value representing an empty cell
    negative: T,
}

/// A struct modelling the possible logical bitwise operations
pub enum BitLogic {
    /// The bitwise AND operation
    And,
    /// The bitwise OR operation
    Or,
    /// The bitwise XOR operation
    Xor,
    /// No bitwise operation
    None,
}

impl<T> Board<T>
where
    T: Copy
        + Clone
        + std::ops::BitAnd<T, Output = T>
        + std::ops::BitOr<Output = T>
        + std::ops::BitXor<T, Output = T>,
{
    /// Create a board filled with false, indicating empty cells.
    /// # Arguments
    /// - `dims` - The width and height of the board as an array of usize's
    /// # Returns
    /// `Array2D<bool>` - The array filled with false
    pub fn new(dims: Coordinate, element: T) -> Self {
        Board {
            board: Array2D::filled_with(element, dims.row, dims.col),
            negative: element,
        }
    }

    /// Create a board from a pre-existing array.
    /// # Arguments
    /// - `array` - The array, representing the internal board state
    /// - `negative` - The value representing an empty cell
    /// # Returns
    /// - `Board<T>` - A board instance
    pub fn from_array(array: &Array2D<T>, negative: T) -> Self {
        Board {
            board: array.clone(),
            negative,
        }
    }

    /// Get a reference to the current board state.
    /// # Returns
    /// - `&Array2D<Bool>` - A reference to the current board state
    pub fn get_array(&self) -> &Array2D<T> {
        &self.board
    }

    /// Get the value of the negative element.
    /// # Returns
    /// - `T` - The negative element
    pub fn get_negative(&self) -> T {
        self.negative
    }

    /// Get the shape of the current board state.
    /// # Returns
    /// - `Coordinate` - The shape of the current board state as a `Coordinate` of [row, col]
    pub fn get_shape(&self) -> Coordinate {
        Coordinate {
            row: self.board.num_rows(),
            col: self.board.num_columns(),
        }
    }

    /// Get the bottom right coordinate of the current board state.
    /// # Returns
    /// - `Coordinate` - The bottom right coordinate, equal to [row - 1, col - 1]
    pub fn get_coords(&self) -> Coordinate {
        self.get_shape() - [1, 1]
    }

    /// Get a slice from an array that is inclusive at the low and exclusive at the high end.
    /// # Arguments
    /// - `coord1` - The lower coordinate for slicing
    /// - `coord2` - The higher coordinate for slicing
    /// # Returns
    /// - `Option<Board<T>>` - A slice of the board if both coordinates are in bounds and `None` otherwise
    pub fn slice(&self, coord1: Coordinate, coord2: Coordinate) -> Option<Board<T>> {
        let coord_low = Coordinate {
            row: min(coord1.row, coord2.row),
            col: min(coord1.col, coord2.col),
        };
        let coord_high = Coordinate {
            row: max(coord1.row, coord2.row),
            col: max(coord1.col, coord2.col),
        };

        let origin = Coordinate::from_array([0, 0]);
        if coord_low.is_within_bounds(origin, self.get_shape())
            && coord_high.is_within_bounds(origin, self.get_shape())
        {
            let dest = coord_high - coord_low;
            let mut row_major = Vec::with_capacity(dest.inner_product());
            for r in coord_low.row..coord_high.row {
                for c in coord_low.col..coord_high.col {
                    row_major.push(*self.get_array().get(r, c).unwrap());
                }
            }
            Some(Board::from_array(
                &Array2D::from_row_major(&row_major, dest.row, dest.col).unwrap(),
                self.get_negative(),
            ))
        } else {
            None
        }
    }

    /// Set a board to a specific value over some range.
    /// # Arguments
    /// - `board` - A muteable reference to an `Array2D` containing some generic
    /// - `value` - A generic of the same type to overwrite the board's values with
    /// - `coord` - The starting coordinate [row, col] as a `Coordinate`
    /// - `dims` - The dimensions of the board range to be set [rows, cols] as a `Coordinate`
    pub fn set_value(&mut self, value: T, coord: Coordinate, dims: Coordinate) {
        // Simple wrapper for set_mask.
        let mask = Array2D::filled_with(value, dims.row, dims.col);
        self.set_mask(&mask, coord);
    }

    /// Set a board to a specific mask over some range without logic.
    /// # Arguments
    /// - `board` - A muteable reference to an `Array2D` containing some generic
    /// - `mask` - A second `Array2D` containing a generic of the same type to overwrite the board's values with
    /// - `coord` - The starting coordinate [row, col] as a `Coordinate`
    pub fn set_mask(&mut self, mask: &Array2D<T>, coord: Coordinate) {
        self._set_mask(mask, coord, &BitLogic::None);
    }

    /// Set a board to a specific mask over some range with AND logic.
    /// # Arguments
    /// - `board` - A muteable reference to an `Array2D` containing some generic
    /// - `mask` - A second `Array2D` containing a generic of the same type to overwrite the board's values with
    /// - `coord` - The starting coordinate [row, col] as a `Coordinate`
    pub fn set_mask_and(&mut self, mask: &Array2D<T>, coord: Coordinate) {
        self._set_mask(mask, coord, &BitLogic::And);
    }

    /// Set a board to a specific mask over some range with OR logic.
    /// # Arguments
    /// - `board` - A muteable reference to an `Array2D` containing some generic
    /// - `mask` - A second `Array2D` containing a generic of the same type to overwrite the board's values with
    /// - `coord` - The starting coordinate [row, col] as a `Coordinate`
    pub fn set_mask_or(&mut self, mask: &Array2D<T>, coord: Coordinate) {
        self._set_mask(mask, coord, &BitLogic::Or);
    }

    /// Set a board to a specific mask over some range with XOR logic.
    /// # Arguments
    /// - `board` - A muteable reference to an `Array2D` containing some generic
    /// - `mask` - A second `Array2D` containing a generic of the same type to overwrite the board's values with
    /// - `coord` - The starting coordinate [row, col] as a `Coordinate`
    pub fn set_mask_xor(&mut self, mask: &Array2D<T>, coord: Coordinate) {
        self._set_mask(mask, coord, &BitLogic::Xor);
    }

    /// Backend for `.set_mask()`, `.set_mask_and()`, `.set_mask_or()` and `.set_mask_xor()` convenience methods.
    fn _set_mask(&mut self, mask: &Array2D<T>, coord: Coordinate, logic: &BitLogic) {
        // Checking if subslice is valid
        // let origin = Coordinate::from_array([0, 0]);
        let mask_size = Coordinate::from_array([mask.num_rows(), mask.num_columns()]);
        // let board_size = Coordinate::from_array([self.get_shape().row, self.get_shape().col]);
        // let dest = coord + mask_size - [1, 1];

        for r in 0..mask_size.row {
            for c in 0..mask_size.col {
                let coord_board = coord + Coordinate::from_array([r, c]);
                self.board
                    .set(
                        coord_board.row,
                        coord_board.col,
                        // Checking logic operation for setting.
                        match logic {
                            BitLogic::And => {
                                *mask.get(r, c).unwrap()
                                    & *self.board.get(coord_board.row, coord_board.col).unwrap()
                            }
                            BitLogic::Or => {
                                *mask.get(r, c).unwrap()
                                    | *self.board.get(coord_board.row, coord_board.col).unwrap()
                            }
                            BitLogic::Xor => {
                                *mask.get(r, c).unwrap()
                                    ^ *self.board.get(coord_board.row, coord_board.col).unwrap()
                            }
                            BitLogic::None => *mask.get(r, c).unwrap(),
                        },
                    )
                    .unwrap();
            }
        }
    }

    /// Compute the logical AND of the current board state with another board state of similar dimensions.
    /// # Arguments
    /// - `array` - Another board state of similar dimensions
    /// # Returns
    /// - `Result<Array2D<T>, Error` - The AND of both board states or an `Error::DimensionMismatch`
    pub fn and(&self, array: &Array2D<T>) -> Result<Board<T>, Error> {
        self._bitlogic(array, &BitLogic::And)
    }

    /// Compute the logical OR of the current board state with another board state of similar dimensions.
    /// # Arguments
    /// - `array` - Another board state of similar dimensions
    /// # Returns
    /// - `Result<Array2D<T>, Error` - The AND of both board states or an `Error::DimensionMismatch`
    pub fn or(&self, array: &Array2D<T>) -> Result<Board<T>, Error> {
        self._bitlogic(array, &BitLogic::Or)
    }

    /// Compute the logical XOR of the current board state with another board state of similar dimensions.
    /// # Arguments
    /// - `array` - Another board state of similar dimensions
    /// # Returns
    /// - `Result<Array2D<T>, Error` - The XOR of both board states or an `Error::DimensionMismatch`
    pub fn xor(&self, array: &Array2D<T>) -> Result<Board<T>, Error> {
        self._bitlogic(array, &BitLogic::Xor)
    }

    /// Backed for `.and()`, `.or()` and `.xor()` convenience methods.
    fn _bitlogic(&self, array: &Array2D<T>, logic: &BitLogic) -> Result<Board<T>, Error> {
        // The array shapes do not match.
        if !self._check_shape_match(array) {
            return Err(Error::DimensionMismatch);
        }
        // Constructing column majors.
        let own_column_major = self.get_array().as_column_major();
        let other_column_major = array.as_column_major();
        let mut logic_column_major = Vec::with_capacity(own_column_major.len());
        match logic {
            // Logical AND of own and other
            BitLogic::And => {
                for (own, other) in own_column_major.iter().zip(other_column_major.iter()) {
                    logic_column_major.push(*own & *other);
                }
            }
            // Logical OR of own and other
            BitLogic::Or => {
                for (own, other) in own_column_major.iter().zip(other_column_major.iter()) {
                    logic_column_major.push(*own | *other);
                }
            }
            // Logical XOR of own and other
            BitLogic::Xor => {
                for (own, other) in own_column_major.iter().zip(other_column_major.iter()) {
                    logic_column_major.push(*own ^ *other);
                }
            }
            // Keep own
            BitLogic::None => logic_column_major = own_column_major,
        }

        // Reconstructing the logical array from the column major.
        let mut clone = Board::from_array(self.get_array(), self.get_negative());
        clone.set_mask(
            &Array2D::from_column_major(
                &logic_column_major,
                self.get_shape().row,
                self.get_shape().col,
            )
            .unwrap(),
            Coordinate::from_array([0, 0]),
        );
        Ok(clone)
    }

    /// Check if the internal board state matches the shape of an external array.
    fn _check_shape_match(&self, array: &Array2D<T>) -> bool {
        self.get_shape() == Coordinate::from_array([array.num_rows(), array.num_columns()])
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::coordinate::Coordinate;
    use array2d::Array2D;

    #[test]
    fn test_set_mask() {
        // Create board with coordinate x:
        //   0 1 2 3
        // 0 f f f f
        // 1 f f X f
        // 2 f f f f
        // Create mask:
        //   0 1
        // 0 t t
        // 1 t f
        // Expect target:
        //   0 1 2 3
        // 0 f f f f
        // 1 f f t t
        // 2 f f t f
        let mut board = Board::new(Coordinate::from_array([3, 4]), false);
        let mask = Array2D::from_row_major(
            &[
                true, true, //
                true, false, //
            ],
            2,
            2,
        )
        .unwrap();
        board.set_mask(&mask, Coordinate::from_array([1, 2]));
        let target = Array2D::from_row_major(
            &[
                false, false, false, false, //
                false, false, true, true, //
                false, false, true, false, //
            ],
            3,
            4,
        )
        .unwrap();
        assert_eq!(board.get_array(), &target);
    }

    #[test]
    #[should_panic]
    fn test_set_mask_error() {
        // Create board:
        //   0 1 2 3 4
        // 0 f f f f f
        // 1 f f f f f
        // 2 X
        // Create mask:
        //   0 1 2
        // 0 f t t
        // 1 t t f
        let mut board = Board::new(Coordinate::from_array([2, 5]), false);
        let mask = Array2D::from_row_major(
            &[
                false, true, true, //
                true, true, false, //
            ],
            2,
            3,
        )
        .unwrap();
        board.set_mask(&mask, Coordinate::from_array([3, 0]))
    }

    #[test]
    fn test_set_value() {
        // Create board with coordinate X:
        //   0 1 2
        // 0 f X f
        // 1 f f f
        // 2 f f f
        // 3 f f f
        // Create mask:
        //   0
        // 0 t
        // 1 t
        // 2 t
        // Expect target:
        //   0 1 2
        // 0 f t t
        // 1 f t t
        // 2 f t t
        // 3 f f f
        let mut board = Board::new(Coordinate::from_array([4, 3]), false);
        board.set_value(
            true,
            Coordinate::from_array([0, 1]),
            Coordinate::from_array([3, 1]),
        );
        let target = Array2D::from_row_major(
            &[
                false, true, false, //
                false, true, false, //
                false, true, false, //
                false, false, false, //
            ],
            4,
            3,
        )
        .unwrap();
        assert_eq!(board.get_array(), &target);
    }

    #[test]
    #[should_panic]
    fn test_set_value_error() {
        // Create board with coordinate X:
        //   0 1
        // 0 X f
        // 1 f f
        // 2 f f
        // 3 f f
        // 4 f f
        // Create mask:
        //   0 1 2
        // 0 t t t
        let mut board = Board::new(Coordinate::from_array([5, 2]), false);
        board.set_value(
            true,
            Coordinate::from_array([0, 0]),
            Coordinate::from_array([1, 3]),
        )
    }
}
