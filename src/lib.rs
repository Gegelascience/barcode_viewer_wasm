use wasm_bindgen::prelude::*;
mod ean_checker;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn check_ean(possible_ean: &str) -> bool {
    return ean_checker::is_correct_ean(possible_ean);
}