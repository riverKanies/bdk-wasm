pub mod bitcoin;
pub mod storage;
pub mod types;
mod utils;

pub use utils::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"bdk-wasm initialized".into());
    set_panic_hook();
}
