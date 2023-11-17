mod utils;

use salad_calculator;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn make_word_salad(first_word: String) -> String {
    let salad_combos: String = salad_calculator::find_word_salads(first_word.as_str())
        .into_iter()
        .collect::<Vec<String>>()
        .join("-");
    
    salad_combos
}