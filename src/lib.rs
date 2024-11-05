mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, bdk-wasm!".into()
}
