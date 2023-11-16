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
  // TODO - do we need an empty root?
  // pub fn create_root() -> WordSalad {
  //   WordSalad {
  //     word: None,
  //     children: HashMap::new(),
  //   }
  // }

  pub fn insert_word(&mut self, word: &String) {
    let salad_layer = self.salad_layer;
    self.children.insert(word.to_string(), WordSalad::create(word, salad_layer + 1));
  }

  pub fn toss_salad(&mut self, word_list: &Vec<String>, solution_set: &mut HashSet<String>) -> WordSalad {
    // TODO - is .clone() the best option here?
    let root_word = self.word.clone().unwrap();
    let used_letters: Vec<char> = root_word.chars().collect();
    println!("SELF {:?}", self);
    for word in word_list.iter() {
      let word_letters: Vec<char> = word.chars().collect();
      let should_insert = match used_letters.as_slice() {
        // satisfy matcher requirements
        [] => false,
        // check that last letter of used_letters matches first letter of current word
        [beginning @ .., last] => !word.contains(beginning) && *last == word_letters[0]
      };
      if should_insert {
        let concat = format!("{}{}", root_word, word);
        WordSalad::insert_word(self, &concat);
      }
    }
    if self.salad_layer > 3 {
      let set = self.word.as_ref().unwrap();
      solution_set.insert(set.to_string());
      return self.to_owned()
    } else {
      let child_nodes = &self.children;
      for node in child_nodes.iter() {
        let mut word_salad = node.1.to_owned();
        WordSalad::toss_salad(&mut word_salad, word_list, solution_set);
      }
      return self.to_owned()
    }
  }

  // pub fn toss_salad(&mut self, root_word: &str, word_list: &Vec<String>) {
  //   let root_letters: Vec<char> = root_word.chars().collect();
  //   println!("ROOT LETTEERS {:?}", root_letters);
  //   let current_word = self.word.as_ref().unwrap();
  //   // handle root level - TODO - reduce branching?
  //   for i in 0..4 {
  //     let child_nodes = self.children.clone();
  //     if i == 0 {
  //       for word in word_list.iter() {
  //         // TODO - is there a better way to do this? substring checking?
  //         let mut should_insert = true;
  //         for letter in root_letters[0..3].iter() {
  //           if word.contains(*letter) {
  //             should_insert = false;
  //           }
  //         }
  //         if should_insert && word.starts_with(root_letters[4]) {
  //           self.insert_word(word)
  //         }
  //       }
  //     } else {
  //       // TODO - debug infinite loop - if it is one
  //       // handle node layers
  //       for mut node in child_nodes {
  //           println!("In here in here word {:?}", node);
  //         // let used_letters = root_word.to_owned().push(node.0);
  //         // for word in word_list.iter() {
  //           // let root_letters: Vec<char> = root_word.chars().collect();
  //           // let mut current_letters: Vec<char> = node.0.to_owned().chars().collect();
  //           // current_letters.pop();
  //           // let used_letters: Vec<char> = current_letters.splice(0..current_letters.len() - 1, root_letters).collect();
  //           // // let mut should_insert = true;
  //           // // for letter in used_letters.iter() {
  //           // //   if word.contains(*letter) {
  //           // //     should_insert = false
  //           // //   }
  //           // // }
  //           // // if should_insert {
  //           // //   node.1.insert_word(word);
  //           // // }
  //         // }
  //       }
  //   }
  //     // println!("ROOT NODE LENGTH {:?}", child_nodes.len());
  //   }
  // }
}

// #[derive(Debug)]
// pub struct WordSalad {
//   root_leaf: WordSaladLeaf
// }

// impl WordSalad {
//   pub fn new() -> WordSalad {
//     WordSalad{
//       root_leaf: WordSaladLeaf::create_root()
//     }
//   }

//   pub fn from(word_list: Vec<String>, input_string: String, parent_salad: WordSalad) -> WordSalad {
//     let mut word_salad = WordSalad::new();

//     for node in parent_salad.iter() {
//       println!("Building leaf...");
//       for word in word_list.iter() {
//         word_salad.insert(word)
//       }
//       println!("Done!");
//     }

//     // println!("Building salad...");
//     // for word in word_list.iter() {
//     //   word_salad.insert(word)
//     // }
//     // println!("Done!");

//     return word_salad
//   }
// }

// is a wordSalad a struct of tries? or is it four leaves of a trie?