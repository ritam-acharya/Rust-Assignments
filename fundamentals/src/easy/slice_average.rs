/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/

pub fn average(values: &[f64]) -> Option<f64> {
    if values.len() < 1 {
      None
    }else {
      let mut ans = 0 as f64;
      for val in values {
        ans += *val;
      }
      let length = values.len() as f64;
      Some(ans / length)
    }
}
