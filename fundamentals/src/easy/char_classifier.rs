/*
  Problem 6: Character Classifier

  Write a function that takes a char and returns a &'static str indicating whether it is
  "alphabetic", "numeric", "whitespace", or "other".

  Run the tests for this problem with:
    cargo test --test char_classifier_test
*/

pub fn classify_char(c: char) -> &'static str {
    let alphabets = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numeric = "0123456789";
    let whitespace = " \t\n";
    if alphabets.contains(c) {
      "alphabetic"
    }else if numeric.contains(c) {
      "numeric"
    } else if whitespace.contains(c) {
      "whitespace"
    }else {
      "other"
    }
}
