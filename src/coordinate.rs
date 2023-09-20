//! A module containig a `Coordinate` struct, modelling an (x,y) position

#![allow(dead_code)]

use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// A basic struct modelling a coordinate as row and a column
pub struct Coordinate {
    /// The coordinate row (vertical, y)
    pub row: usize,
    /// The coordinate column (horizontal, x)
    pub col: usize,
}

impl Coordinate {
    /// Instantiate a `Coordinate` from an coordinate array of [row, col].
    /// # Arguments
    /// - `array` - The coordinate points as a `[usize; 2]` array of [row, col]
    /// # Returns
    /// - `Coordinate` - The `Coordinate` as a [row, col] index
    pub fn from_array(array: [usize; 2]) -> Self {
        Coordinate {
            row: array[0],
            col: array[1],
        }
    }

    /// Compute the inner product of the coordinate as row x column
    pub fn inner_product(&self) -> usize {
        self.row * self.col
    }

    /// Instantiate a `Coordinate` from a row major index.
    /// # Arguments
    /// - `index` - The index as a `usize` in the row major order of elements
    /// - `dims` - The arrays dimensions as a `Coordinate`
    /// # Returns
    /// - `Coordinate` - The `Coordinate` as a [row, col] index
    pub fn from_row_major(index: usize, dims: Coordinate) -> Option<Self> {
        if index <= dims.inner_product() {
            Some(Coordinate {
                row: index / dims.col,
                col: index % dims.col,
            })
        } else {
            None
        }
    }

    /// Instantiate a `Coordinate` from a column major index.
    /// # Arguments
    /// - `index` - The index as a `usize` in the column major order of elements
    /// - `dims` - The arrays dimensions as a `Coordinate`
    /// # Returns
    /// - `Coordinate` - The `Coordinate` as a [row, col] index
    pub fn from_column_major(index: usize, dims: Coordinate) -> Option<Self> {
        if index <= dims.inner_product() {
            Some(Coordinate {
                row: index % dims.row,
                col: index / dims.row,
            })
        } else {
            None
        }
    }

    /// Check if the coordinate is within some boundary
    /// # Arguments
    /// - `lower` - The lower boundary the coordiante should not cross
    /// - `upper` - The higher boundary the coordinate should not cross
    /// # Returns
    /// - `bool` - Whether (`true`) or not (`false`) the coordinate is within the boundary
    pub fn is_within_bounds(&self, lower: Coordinate, upper: Coordinate) -> bool {
        lower.row <= self.row
            && self.row <= upper.row
            && lower.col <= self.col
            && self.col <= upper.col
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

#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;
    use array2d::Array2D;

    #[test]
    fn test_from_row_major() {
        // Create array:
        //  0  1  2  3  4
        //  5  6  7  8  9
        // 10 11 12 13 14
        // Get index 6. Compare get_row_major and get from created Coordinate.
        let array = Array2D::from_iter_row_major(0..15, 3, 5).unwrap();
        let index = 6;
        let coord = Coordinate::from_row_major(
            index,
            Coordinate::from_array([array.num_rows(), array.num_columns()]),
        )
        .unwrap();
        assert_eq!(array.get(coord.row, coord.col), array.get_row_major(index))
    }

    #[test]
    fn test_from_column_major() {
        // Create array:
        // 0 5 10
        // 1 6 11
        // 2 7 12
        // 3 8 13
        // 4 9 14
        // Get index 6. Compare get from created Coordinate and get_column_major.
        let array = Array2D::from_iter_column_major(0..15, 5, 3).unwrap();
        let index = 6;
        let coord = Coordinate::from_column_major(
            index,
            Coordinate::from_array([array.num_rows(), array.num_columns()]),
        )
        .unwrap();
        assert_eq!(
            array.get(coord.row, coord.col),
            array.get_column_major(index)
        )
    }
}
