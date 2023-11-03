mod trie;

// use std::process::exit;
use trie::Trie;
use std::collections::HashSet;

// use std::{
//   fs::File,
//   io::{prelude::*, BufReader},
//   path::Path,
// };

pub fn build_trie(input_string: &str) -> Trie {
  // TODO - input_string?
  // filter word list for size and unused letters
  let word_list = include_str!("../word_list/words_alpha.txt")
    .split("\r\n")
    .map(| str | str.to_string())
    .filter(| ref line | {
      if line.len() != 5 {
        return false;
      }
      let letters: Vec<&str> = line.split("").collect();
      for letter in letters {
        if input_string.contains(letter) {
          return false
        }
      }
      true
     })
     .collect::<Vec<String>>();

    // return Trie of words with unused letters
    Trie::from(word_list)
}

// TODO - build hash set of possible salads, is it a trie?
// build trie (hash map?) from word list according to rules
pub fn find_word_salads(input_string: &str) -> HashSet<String> {
    let _trie = build_trie(input_string);
    let words= HashSet::new();
    words
}