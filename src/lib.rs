use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
