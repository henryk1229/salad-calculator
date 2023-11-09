mod trie;
mod word_salad;

// use std::process::exit;
use trie::Trie;
use word_salad::SaladLayer;
use std::{collections::HashSet, str::Chars};

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

// TODO - build hash set of possible salads, is it a trie?
// build trie (hash map?) from word list according to rules
// TODO - return salad layer or wordsalad?
pub fn find_word_salads(input_string: &str) -> SaladLayer {
    // let _trie = build_trie(input_string);
    // let words= HashSet::new();

    let mut word_salad = SaladLayer::create(&input_string.to_string());

    println!("SALADDDDD {:?}", word_salad);

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

    // make three layers of salad
    for _i in 0..2 {
      SaladLayer::toss_salad(&mut word_salad, input_string, &word_list);
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