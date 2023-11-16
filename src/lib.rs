mod trie;
mod word_salad;

use trie::Trie;
use word_salad::WordSalad;
use std::str::Chars;

// use std::{
//   fs::File,
//   io::{prelude::*, BufReader},
//   path::Path,
// };

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

// TODO - build hash set of possible salads, is it a trie?
// build trie (hash map?) from word list according to rules
// TODO - return salad layer or wordsalad?
pub fn find_word_salads(input_string: &str) -> WordSalad {

    // initialize word_salad with input string
    let mut word_salad = WordSalad::create(&input_string.to_string());

    // build word list
    let word_list = build_word_list();

    // make three layers of salad
    for i in 0..4 {
      WordSalad::toss_salad(&mut word_salad, input_string, &word_list, i);
    };

    // words
  word_salad
}

// create a fn that trims the word list and builds a trie
// loop for three rounds 
// input: 
// mut input_string 'decoysecondword'
// mut word_list 

// return trie (or some oher struct?)