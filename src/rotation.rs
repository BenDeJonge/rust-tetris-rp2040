use array2d::Array2D;
use std::clone::Clone;

/// Transpose a matrix by turning rows into columns or vise versa.
/// # Arguments
/// - `matrix` - A reference to an Array2D of a generic which can be cloned
fn transpose<T: Clone>(matrix: &Array2D<T>) -> Array2D<T> {
    Array2D::from_columns(&matrix.as_rows()).unwrap()
}
/// Rotate a matrix 90 degrees clockwise by transposing and reversing the column order.
/// # Arguments
/// - `matrix` - A reference to an Array2D of a generic which can be cloned
fn rotate_cw<T: Clone>(matrix: &Array2D<T>) -> Array2D<T> {
    let columns: Vec<Vec<T>> = transpose(matrix).as_columns().into_iter().rev().collect();
    Array2D::from_columns(&columns).unwrap()
}

/// Rotate a matrix 90 degrees counterclockwise by transposing and reversing the row order.
/// # Arguments
/// - `matrix` - A reference to an Array2D of a generic which can be cloned
fn rotate_ccw<T: Clone>(matrix: &Array2D<T>) -> Array2D<T> {
    let rows: Vec<Vec<T>> = transpose(matrix).as_rows().into_iter().rev().collect();
    Array2D::from_rows(&rows).unwrap()
}

#[cfg(test)]
mod tests {
    use array2d::Array2D;

    use crate::rotation::{rotate_ccw, rotate_cw, transpose};

    #[test]
    fn test_transpose() {
        // Make matrix:
        // [ 1, 2, 3 ]
        // [ 4, 5, 6 ]
        // Transpose to:
        // [ 1, 4 ]
        // [ 2, 5 ]
        // [ 3, 6 ]
        let columns1 = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let m1 = Array2D::from_columns(&columns1).unwrap();
        let columns2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m2 = Array2D::from_columns(&columns2).unwrap();
        assert_eq!(transpose(&m1), m2);
        assert_eq!(m1, transpose(&m2));
    }

    #[test]
    fn test_rotate_cw() {
        // Make matrix:
        // [ 1, 2, 3 ]
        // [ 4, 5, 6 ]
        // Rotate to:
        // [ 4, 1 ]
        // [ 5, 2 ]
        // [ 6, 3 ]
        let columns1 = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let m1 = Array2D::from_columns(&columns1).unwrap();

        let columns2 = vec![vec![4, 5, 6], vec![1, 2, 3]];
        let m2 = Array2D::from_columns(&columns2).unwrap();

        assert_eq!(rotate_cw(&m1), m2);
    }

    #[test]
    fn test_rotate_ccw() {
        // Make matrix:
        // [ 4, 1 ]
        // [ 5, 2 ]
        // [ 6, 3 ]
        // Rotate to:
        // [ 1, 2, 3 ]
        // [ 4, 5, 6 ]
        let columns1 = vec![vec![4, 5, 6], vec![1, 2, 3]];
        let m1 = Array2D::from_columns(&columns1).unwrap();

        let columns2 = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let m2 = Array2D::from_columns(&columns2).unwrap();

        assert_eq!(rotate_ccw(&m1), m2);
    }
}
