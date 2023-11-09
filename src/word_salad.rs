use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SaladLayer {
  word: Option<String>,
  children: HashMap<String, SaladLayer>,
}

impl SaladLayer {
  pub fn create(word: &String) -> SaladLayer {
    SaladLayer {
      word: Some(word.to_string()),
      children: HashMap::new()
    }
  }
  // TODO - do we need an empty root?
  // pub fn create_root() -> SaladLayer {
  //   SaladLayer {
  //     word: None,
  //     children: HashMap::new(),
  //   }
  // }

  pub fn insert_word(&mut self, word: &String) {
    // let word = word.clone().to_string();
    self.children.insert(word.to_string(), SaladLayer::create(word));
  }

  pub fn toss_salad(&mut self, root_word: &str, word_list: &Vec<String>) {
    // handle root level - TODO - reduce branching?
    if self.word.as_ref().unwrap() == root_word {
      for word in word_list.iter() {
          SaladLayer::insert_word(self, word)
      }
      return
    };
    // handle node layers
    let nodes = self.children.clone();
    println!("NODESSSSS {:?} ROOT {:?}", nodes, root_word);
    // let used_letters: Vec<char> = root_word.chars().collect();
    for node in nodes {
      // let used_letters = root_word.to_owned().push(node.0);
      println!("In here node {:?}", node);
      for word in word_list.iter() {
        println!("In here in here word {}", word);
        if !root_word.contains(word) && !node.0.contains(word) {
          SaladLayer::insert_word(self, word);
        }
      }
    }
  }
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