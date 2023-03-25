use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type EntityCluster;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EntityCluster;
}
