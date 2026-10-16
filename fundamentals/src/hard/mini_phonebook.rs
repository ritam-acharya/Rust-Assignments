/*
  Problem 35: Mini Phonebook

  Build a Phonebook struct backed by a HashMap<String, String> (name -> phone).
  Implement methods new, add, lookup, and remove.
  Also implement the Display trait to print all entries sorted by name, one per line
  as "Name: Phone".

  Run the tests for this problem with:
    cargo test --test mini_phonebook_test
*/

use std::collections::HashMap;
use std::fmt;

pub struct Phonebook {
    pub entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        Phonebook {
            entries: HashMap::new()
        }
    }

    pub fn add(&mut self, name: &str, phone: &str) {
        self.entries.insert(name.to_string(), phone.to_string());
    }

    pub fn lookup(&self, name: &str) -> Option<&String> {
        self.entries.get(name)
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let resp = self.entries.remove(name);
        match resp {
            Some(_) => true,
            None => false
        }
    }
}

impl fmt::Display for Phonebook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_entries: Vec<(&String, &String)> = self.entries.iter().collect();
        sorted_entries.sort_by_key(|&(name, _phone)| name);
        for (name, phone) in sorted_entries {
            write!(f, "{}: {}\n", name, phone)?;
        }
        Ok(())
    }
}
