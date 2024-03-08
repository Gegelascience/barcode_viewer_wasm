use wasm_bindgen::prelude::*;
mod ean_checker;
mod barcode_data;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

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
    ean_checker::is_correct_ean(possible_ean)
}

#[wasm_bindgen]
pub fn get_ean_13_barcode_data(possible_ean: &str) -> String {
    if ean_checker::is_correct_ean(possible_ean) {
        return barcode_data::calculate_barcode_ean13(possible_ean);
    }
    String::from("")
    
}


// webgl