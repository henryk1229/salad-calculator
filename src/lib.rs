mod trie;
mod word_salad;

use trie::Trie;
use word_salad::WordSalad;
use std::str::Chars;

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
  let word_list = include_str!("../word_list/words_alpha.txt")
    .split("\r\n")
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

pub fn find_word_salads(input_string: &str) -> WordSalad {

    // initialize word_salad with input string
    let mut word_salad = WordSalad::create(&input_string.to_string(), true);

    // build word list
    let word_list = build_word_list();

    // TODO - recurse / iterate
    word_salad.toss_salad(&word_list);

  // return solution set
  word_salad
}