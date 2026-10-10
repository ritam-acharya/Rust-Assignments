/*
  Problem 15: Parse Integer with Result

  Write a function that takes a &str and attempts to parse it into an i32.
  Return Ok(value) on success, or Err(String) with a descriptive error message on failure.

  Run the tests for this problem with:
    cargo test --test parse_int_test
*/

pub fn parse_int(s: &str) -> Result<i32, String> {
    let resp = s.parse::<i32>();
    match resp {
      Ok(val) => Ok(val),
      Err(_) => Err(String::from("Can't parse into i32 format"))
    }
}
