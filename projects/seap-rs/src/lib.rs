mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, seap-rs!");
}

//pack for number in one  u16,u16,u16,u16 -> u64
#[wasm_bindgen]
pub fn pack(a: u16, b: u16, c: u16, d: u16) -> f64 {
    let mut result: u64 = 0;
    result |= a as u64;
    result |= (b as u64) << 16;
    result |= (c as u64) << 32;
    result |= (d as u64) << 48;
    result as f64
}
