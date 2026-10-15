/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    if s.is_empty() {
      return s.to_string();
    }

    let mut compressed = String::new();
    let mut count = 0;
    let chars_vec: Vec<char> = s.chars().collect();
    let mut last_char = chars_vec[0];
    for ch in chars_vec {
      if ch == last_char {
        count += 1;
      } else {
        compressed.push(last_char);
        compressed.push_str(&count.to_string());
        last_char = ch;
        count = 1;
      }
    }
    compressed.push(last_char);
    compressed.push_str(&count.to_string());

    if compressed.len() < s.len() {
      compressed
    } else {
      s.to_string()
    }

}
