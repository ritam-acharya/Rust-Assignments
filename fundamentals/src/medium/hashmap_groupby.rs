/*
  Problem 32: HashMap — Group By First Letter

  Write a function that takes a Vec<String> and returns a HashMap<char, Vec<String>>
  where words are grouped by their first character (lowercase).
  Ignore empty strings.

  Run the tests for this problem with:
    cargo test --test hashmap_groupby_test
*/

use std::collections::HashMap;

pub fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut ans: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        if word.len() >= 1 {
            if ans.contains_key(&word.chars().next().unwrap()) {
                ans.get_mut(&word.chars().next().unwrap()).unwrap().push(word);
            }else {
                let mut new_vec: Vec<String> = Vec::new();
                new_vec.push(word.clone());
                ans.insert(word.chars().next().unwrap(), vec![word]);
            }
      }
    }
    ans
}
