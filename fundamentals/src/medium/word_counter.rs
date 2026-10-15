/*
  Problem 19: Word Counter

  Write a function that takes a &str and returns a HashMap<String, usize>
  where each key is a lowercase word and each value is the number of occurrences.
  Split on whitespace and convert to lowercase.

  Run the tests for this problem with:
    cargo test --test word_counter_test
*/

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut count_map: HashMap<String, usize> = HashMap::new();
    for word in words {
      let word = word.to_lowercase();
      count_map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }
    count_map
}
