mod board;
mod color;
mod coordinate;
mod rotation;
mod tetrominoes;

use coordinate::Coordinate;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

fn main() {
    let dims = Coordinate::from_array([HEIGHT, WIDTH]);
    let mut _board = board::create_board(dims);
}
