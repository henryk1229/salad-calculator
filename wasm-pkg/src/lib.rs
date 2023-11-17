mod utils;

use salad_calculator;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn make_word_salad(first_word: String) -> JsValue {
    let solution_set = salad_calculator::find_word_salads(first_word.as_str());
    
    serde_wasm_bindgen::to_value(&solution_set).unwrap()
}