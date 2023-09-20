//! A binary crate running a tetris clone on a RP2040 microcontroller

#![warn(missing_docs)]

pub mod board;
pub mod color;
pub mod coordinate;
pub mod gravity;
pub mod rotation;
pub mod tetrominoes;

use board::Board;
use coordinate::Coordinate;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

fn main() {
    let dims = Coordinate::from_array([HEIGHT, WIDTH]);
    let mut _board = Board::new(dims, false);
}
