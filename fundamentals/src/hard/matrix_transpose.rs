/*
  Problem 34: Matrix Transpose

  Write a function that takes a Vec<Vec<i32>> representing a matrix and returns its transpose.
  The transpose of a matrix swaps rows and columns. Assume the input is a valid rectangular
  matrix (all rows have the same length). Return an empty vec for empty input.

  Run the tests for this problem with:
    cargo test --test matrix_transpose_test
*/

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return matrix;
    }

    let cols = matrix[0].len();
    let rows = matrix.len();
    let mut ans: Vec<Vec<i32>> = Vec::new();

    for col in 0..cols {
        let mut new_row: Vec<i32> = Vec::new();
        for row in 0..rows {
            new_row.push(matrix[row][col]);
        }
        ans.push(new_row);
    }

    ans
}
