use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type DataSourceClock;

    #[wasm_bindgen(constructor)]
    pub fn new() -> DataSourceClock;
}
