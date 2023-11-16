mod trie;
mod word_salad;

use trie::Trie;
use word_salad::WordSalad;
use std::str::Chars;
use std::collections::HashSet;

pub fn build_trie(input_string: &str) -> Trie {
  // TODO - input_string?
  // filter word list for size and unused letters
  let word_list = include_str!("../word_lists/anagram_dictionary.txt")
    .split("\r\n")
    .map(| str | str.to_string())
    .filter(| ref line | {
      if line.len() != 5 {
        return false;
      }
      let letters: Chars = line.chars();
      for letter in letters {
        if input_string.contains(letter) {
          return false
        }
      }
      true
      // TODO - filter for letter uniqueness?
     })
     .collect::<Vec<String>>();

    println!("input {}", word_list.len());

    // return Trie of words with unused letters
    Trie::from(word_list)
}

// build 5 letter word list
pub fn build_word_list() -> Vec<String> {
  let word_list = include_str!("../word_lists/anagram_dictionary.txt")
    .split("\n")
    .map(| str | str.to_string())
    .filter(| ref line | {
      if line.len() != 5 {
        return false
      }
      true
    })
    .collect::<Vec<String>>();
  word_list
}

pub fn find_word_salads(input_string: &str) -> HashSet<String> {

  // initialize word_salad with input string
  let mut word_salad = WordSalad::create(&input_string.to_string(), 0);

  // build word list
  let word_list = build_word_list();

  let mut solution_set = HashSet::new();

  // TODO - simplify approach
  WordSalad::toss_salad(&mut word_salad, &word_list, &mut solution_set);

  solution_set
}