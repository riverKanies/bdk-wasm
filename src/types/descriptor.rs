use wasm_bindgen::prelude::wasm_bindgen;

/// Pair of descriptors for external and internal keychains
#[wasm_bindgen]
#[derive(Debug)]
pub struct DescriptorPair {
    /// External descriptor
    external: String,
    /// Internal descriptor
    internal: String,
}

#[wasm_bindgen]
impl DescriptorPair {
    #[wasm_bindgen(constructor)]
    pub fn new(external: String, internal: String) -> Self {
        DescriptorPair { external, internal }
    }

    #[wasm_bindgen(getter)]
    pub fn internal(&self) -> String {
        self.internal.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn external(&self) -> String {
        self.external.clone()
    }
}
