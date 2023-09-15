#![allow(dead_code)]

use crate::coordinate::Coordinate;
use array2d::{Array2D, Error};

/// Create a board filled with false, indicating empty cells.
/// # Arguments
/// - `dims` - The width and height of the board as an array of usize's.
/// # Returns
/// `Array2D<bool>` - The array filled with false
pub fn create_board(dims: Coordinate) -> Array2D<bool> {
    Array2D::filled_with(false, dims.row, dims.col)
}

/// TODO: how to set tetromino:
/// - bind tetromino array by top-left coordinate
/// - get all columns of the tetromino array
/// - set all columns of the array in column major order (set_column_major), row major would loop over to next row
/// - monitor the height of the tetromino array. Halt it when at its own height above the lowest row
/// - monitor logical of tetromino and board array, if ever true: stop the tetromino
/// - monitor the previous position of the tetromino and set it to false to avoid trues along the taken path

/// Set a board to a specific value over some range.
/// # Arguments
/// - `board` - A muteable reference to an `Array2D` containing some generic
/// - `value` - A generic of the same type to overwrite the board's values with
/// - `coord` - The starting coordinate [row, col] as a `Coordinate`
/// - `dims` - The dimensions of the board range to be set [rows, cols] as a `Coordinate`
pub fn set_value<T: Copy>(
    board: &mut Array2D<T>,
    value: T,
    coord: Coordinate,
    dims: Coordinate,
) -> Result<(), Error> {
    // Simple wrapper for set_mask.
    let mask = Array2D::filled_with(value, dims.row, dims.col);
    set_mask(board, &mask, coord)
}

/// Set a board to a specific mask over some range.
/// # Arguments
/// - `board` - A muteable reference to an `Array2D` containing some generic
/// - `mask` - A second `Array2D` containing a generic of the same type to overwrite the board's values with
/// - `coord` - The starting coordinate [row, col] as a `Coordinate`
pub fn set_mask<T: Copy>(
    board: &mut Array2D<T>, // [3, 4] -> [2, 3]
    mask: &Array2D<T>,      // [2, 2] -> [1, 1]
    coord: Coordinate,      // [1, 2] -> coord + mask = [2, 3]
) -> Result<(), Error> {
    // Checking if subslice is valid
    let mask_size = Coordinate::from_array([mask.num_rows(), mask.num_columns()]);
    let board_size = Coordinate::from_array([board.num_rows(), board.num_columns()]);
    let dest = coord + mask_size - [1, 1];
    if dest.is_within_bounds(board_size) {
        for r in 0..mask_size.row {
            for c in 0..mask_size.col {
                let coord_board = coord + Coordinate::from_array([r, c]);
                board.set(coord_board.row, coord_board.col, *mask.get(r, c).unwrap())?;
            }
        }
        return Ok(());
    };
    Err(Error::IndicesOutOfBounds(dest.row, dest.col))
}

#[cfg(test)]
mod tests {
    use crate::board::{create_board, set_mask, set_value};
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
        let mut board = create_board(Coordinate::from_array([3, 4]));
        let mask = Array2D::from_row_major(
            &[
                true, true, //
                true, false, //
            ],
            2,
            2,
        )
        .unwrap();
        set_mask(&mut board, &mask, Coordinate::from_array([1, 2]))
            .expect("Coordinates where within bounds");
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
        assert_eq!(board, target);
    }

    #[test]
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
        let mut board = create_board(Coordinate::from_array([2, 5]));
        let mask = Array2D::from_row_major(
            &[
                false, true, true, //
                true, true, false, //
            ],
            2,
            3,
        )
        .unwrap();
        assert!(set_mask(&mut board, &mask, Coordinate::from_array([3, 0])).is_err())
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
        let mut board = create_board(Coordinate::from_array([4, 3]));
        set_value(
            &mut board,
            true,
            Coordinate::from_array([0, 1]),
            Coordinate::from_array([3, 1]),
        )
        .expect("Coordinates where within bounds");
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
        assert_eq!(board, target);
    }

    #[test]
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
        let mut board = create_board(Coordinate::from_array([5, 2]));
        assert!(set_value(
            &mut board,
            true,
            Coordinate::from_array([0, 0]),
            Coordinate::from_array([1, 3])
        )
        .is_err())
    }
}
