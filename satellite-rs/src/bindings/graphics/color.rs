use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Color;

    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color;
}
