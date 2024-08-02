mod word_salad;

use word_salad::WordSalad;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

const NON_LETTER_CHARS: [char; 4] = ['!', '.', '-', '\''];

// build 5 letter word list to pick initial word and generate word_salads
pub fn build_word_list() -> Vec<&'static str> {
  let word_list = include_str!("../word_lists/salad_dictionary.txt")
    .split('\n')
    .filter(| line | {
      if line.len() < 5 {
        return false
      }
      true
    })
    .collect::<Vec<&str>>();
  word_list
}

// helper fn to find 5-letter words from large anagram list
pub fn build_salad_dictionary() -> Vec<&'static str> {
  let word_list = include_str!("../word_lists/anagram_dictionary.txt")
    .split('\n')
    .filter(| line | {
      if line.len() != 5 {
        return false
      }
      // ensure unique letters in each word
      let chars: Vec<char> = line.chars().collect();
      let mut set: HashSet<char> = HashSet::new();
      for char in chars {
        // if the word contains forbidden characters, explicitly prevent set.len == line.len below
        if !NON_LETTER_CHARS.contains(&char) {
          set.insert(char);
        }
      };
      set.len() == line.len()
    })
    .collect::<Vec<&str>>();
  word_list
}

// init fn to build 5-letter word dictionary
pub fn write_dictionary_file() -> () {
  // build word_list
  let word_list = build_salad_dictionary();

  println!("Creating dictionary file");
  // create file
  let file = File::create("salad_dictionary.txt").unwrap();

  // write to file
  println!("Writing word list to dictionary");
  let mut writer = BufWriter::new(file);

  for word in word_list {
    writer.write_all(format!("{word}\n").as_bytes()).expect("Could not write dictionary :-(");
  }

  println!("Dictionary written!");
}

// pub fn  pick_word() -> String {
//   // build word list

//   // pick word at random

//   // return it
//   "Todo".to_string()
// }

pub fn find_word_salads(input_string: &str) -> HashSet<String> {

  println!("Creating a WordSalad from {input_string}");
  // initialize word_salad with input string
  let mut word_salad = WordSalad::create(&input_string.to_string(), 0);

  // build word list
  println!("Building the word list...");
  let word_list = build_word_list();
  println!("Done!");

  // initialize solution set
  let mut solution_set = HashSet::new();

  // TODO - simplify approach?
  println!("Tossing the salad...");
  WordSalad::toss_salad(&mut word_salad, &word_list, &mut solution_set);
  println!("Done!");

  solution_set
}