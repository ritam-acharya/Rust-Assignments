/*
  Problem 5: Celsius to Fahrenheit

  Write a function that converts a temperature from Celsius (f64) to Fahrenheit using the formula F = C * 9/5 + 32.
  Return the result as f64.

  Run the tests for this problem with:
    cargo test --test celsius_fahrenheit_test
*/

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let multiply_factor: f64 = 9.0 / 5.0;
    let left_part: f64 = c * multiply_factor;
    let ans: f64 = left_part + 32.0;
    ans 
}
