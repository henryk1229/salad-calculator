use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct WordSalad {
  word: Option<String>,
  salad_layer: u8,
  children: HashMap<String, WordSalad>,
}

impl WordSalad {
  pub fn create(word: &String, salad_layer: u8) -> WordSalad {
    WordSalad {
      word: Some(word.to_string()),
      salad_layer,
      children: HashMap::new()
    }
  }

  pub fn insert_word(&mut self, word: &String) {
    let salad_layer = self.salad_layer;
    self.children.insert(word.to_string(), WordSalad::create(word, salad_layer + 1));
  }

  pub fn toss_salad(&mut self, word_list: &Vec<&str>, solution_set: &mut HashSet<String>) {
    // TODO - is .clone() the best option here?
    let root_word = self.word.clone().unwrap();
    let used_letters: Vec<char> = root_word.chars().collect();
    for word in word_list.iter() {
      let word_letters: Vec<char> = word.chars().collect();
      let should_insert = match used_letters.as_slice() {
        // satisfy matcher requirements
        [] => false,
        // check that last letter of used_letters matches first letter of current word
        [beginning @ .., last] => !word.contains(beginning) && *last == word_letters[0]
      };
      if should_insert {
        let concat = format!("{},{}", root_word, word);
        WordSalad::insert_word(self, &concat);
      }
    }
    if self.salad_layer >= 3 {
      let set = self.word.as_ref().unwrap();
      solution_set.insert(set.to_string());
    } else {
      let child_nodes = &self.children;
      for node in child_nodes.iter() {
        let mut word_salad = node.1.to_owned();
        WordSalad::toss_salad(&mut word_salad, word_list, solution_set);
      }
    }
  }
}