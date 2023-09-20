#![allow(dead_code)]
use std::iter::Iterator;

use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::tetrominoes::Tetromino;

/// Check if a tetromino is within the bounds of the board at a certain coordinate.
/// # Arguments
/// - `coord` - The position of the top-left element of the tetromino mask on the board
/// - `board` - A muteable reference the `Board` object
/// - `tetromino` - A reference to the `Tetromino` object
/// # Returns
/// - `bool` - Whether (`true`) or not (`false`) the tetromino is within the bounds of the board
pub fn tetromino_is_in_bounds<T>(
    coord: Coordinate,
    board: &Board<T>,
    tetromino: &Tetromino<T>,
) -> bool
where
    T: Copy
        + Clone
        + std::ops::BitAnd<T, Output = T>
        + std::ops::BitOr<T, Output = T>
        + std::ops::BitXor<T, Output = T>,
{
    (coord + tetromino.get_shape())
        .is_within_bounds(Coordinate::from_array([0, 0]), board.get_shape())
}

/// Check if a tetromino reached the bottom row of the board at a certain coordinate.
/// # Arguments
/// - `coord` - The position of the top-left element of the tetromino mask on the board
/// - `board` - A muteable reference the `Board` object
/// - `tetromino` - A reference to the `Tetromino` object
/// # Returns
/// - `bool` - Whether (`true`) or not (`false`) the tetromino reached the bottom of the board
pub fn tetromino_reached_bottom<T>(
    coord: Coordinate,
    board: &Board<T>,
    tetromino: &Tetromino<T>,
) -> bool
where
    T: Copy
        + Clone
        + std::ops::BitAnd<T, Output = T>
        + std::ops::BitOr<T, Output = T>
        + std::ops::BitXor<T, Output = T>,
{
    // TODO: check if > or >=. Ideally some mobility until trying to sink out of view.
    (coord + tetromino.get_shape()).row >= board.get_shape().row
}

/// Check if a tetromino hit another block.
/// # Arguments
/// - `coord` - The position of the top-left element of the tetromino mask on the board
/// - `board` - A muteable reference the `Board` object
/// - `tetromino` - A reference to the `Tetromino` object
/// # Returns
/// - `bool` - Whether (`true`) or not (`false`) the tetromino hit another block
pub fn tetromino_hit<T>(coord: Coordinate, board: &Board<T>, tetromino: &Tetromino<T>) -> bool
where
    T: Copy
        + Clone
        + std::cmp::PartialEq<bool>
        + std::ops::BitAnd<T, Output = T>
        + std::ops::BitOr<T, Output = T>
        + std::ops::BitXor<T, Output = T>,
{
    let slice_ = board.slice(coord, coord + tetromino.get_shape());
    let mut slice = slice_.unwrap();
    slice.set_mask_and(tetromino.get_mask(), Coordinate::from_array([0, 0]));
    // let arr = slice.get_array();
    // let mut row_major = arr.as_row_major();
    // let mut iter = row_major.iter_mut();
    // let any = iter.any(|el| *el == true);
    // any
    let any = slice
        .get_array()
        .elements_row_major_iter()
        .any(|&el| el == true);
    any
}

/// Set the array of a `Tetromino` on the interal board state of the `Board`.
/// # Arguments
/// - `coord` - The position of the top-left element of the tetromino mask on the board
/// - `value` - The value to set the tetromino mask to in the board, as the same generic in `Tetromino`
/// - `board` - A muteable reference the `Board` object
/// - `tetromino` - A reference to the `Tetromino` object
/// # Returns
/// - `Board<T>` - The internal board state after correctly setting the mask
/// - `Error::IndicesOutOfBounds` - Raises an error when the tetromino mask cannot be set at that coordinate
// pub fn set_tetromino<T>(
//     coord: Coordinate,
//     board: &Board<T>,
//     tetromino: &Tetromino<T>,
// ) -> Result<Board<T>, Error>
// where
//     T: Copy
//         + Clone
//         + std::ops::BitXor<T, Output = T>
//         + std::ops::BitAnd<T, Output = T>
//         + std::ops::BitOr<T, Output = T>,
// {
//     let mut new = Board::from_array(board.get_array(), board.get_negative());
//     new.set_mask(tetromino.get_mask(), coord);
//     board.or(new.get_array())
// }

/// Drop a tetromino to the next row.
/// # Arguments
/// - `coord` - The position of the top-left element of the tetromino mask on the board
/// - `board` - A muteable reference the `Board` object with some lifetime `'a`
/// - `tetromino` - A reference to the `Tetromino` object
/// # Returns
/// - `Result<&`a mut Board<T>, Error> - A muteable reference to the updated board state with the same lifetime `'a` or
/// an `array2d::Error::IndicesOutOfBounds` error.
// pub fn drop_tetromino<T>(coord: Coordinate, board: &mut Board<T>, tetromino: &Tetromino<T>)
// where
//     T: Copy
//         + Clone
//         + std::ops::BitAnd<T, Output = T>
//         + std::ops::BitOr<T, Output = T>
//         + std::ops::BitXor<T, Output = T>,
// {
//     let mut row_major = Vec::with_capacity(tetromino.get_mask().num_elements());
//     for _ in 0..tetromino.get_shape().col {
//         row_major.push(board.get_negative());
//     }
//     for el in tetromino.get_mask().as_row_major() {
//         row_major.push(el);
//     }
//     let mask_zero_row = Array2D::from_row_major(
//         &row_major,
//         tetromino.get_shape().row + 1,
//         tetromino.get_shape().col,
//     )
//     .unwrap();
//     board.set_mask(&mask_zero_row, coord);
// }

#[cfg(test)]

mod tests {

    use super::{tetromino_hit, tetromino_reached_bottom};
    use crate::{
        board::Board,
        coordinate::Coordinate,
        // gravity::drop_tetromino,
        tetrominoes::{Tetromino, TetrominoShape},
    };
    use array2d::Array2D;
    use test_case::test_case;

    // TODO: define macro to expand into different test_case
    // https://stackoverflow.com/questions/65773658/rust-macro-to-generate-multiple-individual-tests

    #[test_case(TetrominoShape::J)]
    #[test_case(TetrominoShape::L)]
    #[test_case(TetrominoShape::S)]
    #[test_case(TetrominoShape::T)]
    #[test_case(TetrominoShape::Z)]
    fn test_bottom_2x3(shape: TetrominoShape) {
        // For the 2x3 tetrominos, rotate 5 times and see if reached the bottom only errors when 3 high.
        let coord = Coordinate::from_array([2, 0]);
        let mut tetromino = Tetromino::from(shape);
        let board = Board::new(Coordinate { row: 5, col: 6 }, false);
        for rot in 0..5 {
            tetromino.rotate_cw();
            match rot % 2 == 0 {
                true => assert!(tetromino_reached_bottom(coord, &board, &tetromino)),
                false => assert!(!tetromino_reached_bottom(coord, &board, &tetromino)),
            }
        }
    }

    #[test_case(TetrominoShape::J)]
    #[test_case(TetrominoShape::L)]
    #[test_case(TetrominoShape::S)]
    #[test_case(TetrominoShape::T)]
    #[test_case(TetrominoShape::Z)]
    fn test_hit_2x3(shape: TetrominoShape) {
        // For the 2x3 tetrominos, rotate 5 times and see if reached the bottom only errors when 3 high.
        let coord = Coordinate::from_array([1, 0]);
        let mut tetromino = Tetromino::from(shape);
        let board = Board::from_array(
            &Array2D::from_row_major(
                &[
                    false, false, false, // . . .
                    false, false, false, // . . .
                    false, false, false, // . . .
                    true, true, true, //    x x x
                    true, true, true, //    x x x
                ],
                5,
                3,
            )
            .unwrap(),
            false,
        );
        for rot in 0..5 {
            tetromino.rotate_cw();
            match rot % 2 == 0 {
                true => assert!(tetromino_hit(coord, &board, &tetromino)),
                false => assert!(!tetromino_hit(coord, &board, &tetromino)),
            }
        }
    }

    // #[test_case(TetrominoShape::I)]
    // #[test_case(TetrominoShape::J)]
    // #[test_case(TetrominoShape::L)]
    // #[test_case(TetrominoShape::O)]
    // #[test_case(TetrominoShape::S)]
    // #[test_case(TetrominoShape::T)]
    // #[test_case(TetrominoShape::Z)]
    // fn test_setting(shape: TetrominoShape) {
    //     // For all tetrominos, rotate 5 times and see if board allow placement.
    //     let coord = Coordinate::from_array([0, 0]);
    //     let mut tetromino = Tetromino::from(shape);
    //     let board = Board::new(Coordinate { row: 10, col: 5 }, false);
    //     for _ in 0..5 {
    //         tetromino.rotate_cw();
    //         assert!(set_tetromino(coord, &board, &tetromino).is_ok())
    //     }
    // }

    // #[test_case(TetrominoShape::I)]
    // #[test_case(TetrominoShape::J)]
    // #[test_case(TetrominoShape::L)]
    // #[test_case(TetrominoShape::O)]
    // #[test_case(TetrominoShape::S)]
    // #[test_case(TetrominoShape::T)]
    // #[test_case(TetrominoShape::Z)]
    // #[should_panic]
    // fn test_setting_error(shape: TetrominoShape) {
    //     // For all tetrominos, rotate 5 times and see if board errors on placement.
    //     let coord = Coordinate::from_array([5, 0]);
    //     let mut tetromino = Tetromino::from(shape);
    //     let board = Board::new(Coordinate { row: 5, col: 5 }, false);
    //     for _ in 0..5 {
    //         tetromino.rotate_cw();
    //         assert!(set_tetromino(coord, &board, &tetromino).is_err())
    //     }
    // }

    // #[test_case(TetrominoShape::J)]
    // #[test_case(TetrominoShape::L)]
    // #[test_case(TetrominoShape::S)]
    // #[test_case(TetrominoShape::T)]
    // #[test_case(TetrominoShape::Z)]
    // #[should_panic]
    // fn test_setting_2x3(shape: TetrominoShape) {
    //     // For the 2x3 tetrominos, rotate 5 times and see if board only errors when 3 wide.
    //     let coord = Coordinate::from_array([0, 3]);
    //     let mut tetromino = Tetromino::from(shape);
    //     let board = Board::new(Coordinate { row: 6, col: 5 }, false);
    //     for rot in 1..6 {
    //         tetromino.rotate_cw();
    //         match rot % 2 == 1 {
    //             true => assert!(set_tetromino(coord, &board, &tetromino).is_ok()),
    //             false => assert!(set_tetromino(coord, &board, &tetromino).is_err()),
    //         }
    //     }
    // }

    // #[test]
    // fn test_drop() {
    //     let coord = Coordinate::from_array([1, 1]);
    //     let mut board = Board::new(Coordinate::from_array([3, 6]), false);
    //     let tetromino = Tetromino::from(TetrominoShape::I);
    //     board.set_mask(tetromino.get_mask(), coord);
    //     // Checking if tetromino is set correctly.
    //     assert_eq!(
    //         board.get_array(),
    //         &Array2D::from_row_major(
    //             &[
    //                 false, false, false, false, false, false, // . . . . . .
    //                 false, true, true, true, true, false, //     . x x x x .
    //                 false, false, false, false, false, false, // . . . . . .
    //             ],
    //             3,
    //             6,
    //         )
    //         .unwrap()
    //     );

    //     // Checking if tetromino is dropped correctly.
    //     drop_tetromino(coord, &mut board, &tetromino);
    //     assert_eq!(
    //         board.get_array(),
    //         &Array2D::from_row_major(
    //             &[
    //                 false, false, false, false, false, false, // . . . . . .
    //                 false, false, false, false, false, false, // . . . . . .
    //                 false, true, true, true, true, false, //     . x x x x .
    //             ],
    //             3,
    //             6,
    //         )
    //         .unwrap()
    //     );
    // }
}
