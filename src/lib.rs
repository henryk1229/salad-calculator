mod trie;

// use std::collections::HashSet;
// use std::process::exit;
use trie::Trie;

// use std::{
//   fs::File,
//   io::{prelude::*, BufReader},
//   path::Path,
// };

// TODO - use input
pub fn find_salads(_input_string: &str) -> Trie {
  // let unused_letters = input_string.trim();
  // validate_letters(unused_letters);

  let word_list = include_str!("../word_list/words_alpha.txt")
    .split("\r\n")
    .map(| str | str.to_string())
    .filter(| ref line | {
      if line.len() != 5 {
        return false;
      }
      true
     })
     .collect::<Vec<String>>();

    let trie = Trie::from(word_list);

    trie
}