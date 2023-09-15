use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    /// A basic struct modelling a coordinate as row and a column
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    /// Instantiate a `Coordinate` from an coordinate array of [row, col].
    pub fn from_array(array: [usize; 2]) -> Self {
        Coordinate {
            row: array[0],
            col: array[1],
        }
    }

    /// Check if the coordinate is within some boundary
    /// # Arguments
    /// - `coord` - The coordinate to evaluation
    /// - `bounds` - The boundary the coordinate should not cross
    /// # Returns
    /// - `bool` - Whether (`true`) or not (`false`) the coordinate is within the boundary
    pub fn is_within_bounds(&self, bounds: Coordinate) -> bool {
        (self.row <= bounds.row) && (self.col <= bounds.col)
    }
}

/// Overloading + and - operators for other Coordinate
impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

/// Overloading + and - operators for array representing a coordinate
impl ops::Add<[usize; 2]> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: [usize; 2]) -> Self::Output {
        self + Coordinate::from_array(rhs)
    }
}

impl ops::Sub<[usize; 2]> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: [usize; 2]) -> Self::Output {
        self - Coordinate::from_array(rhs)
    }
}
