pub mod bitcoin;
pub mod types;
mod utils;

pub use utils::*;

// Use `wee_alloc` as the global allocator for WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
