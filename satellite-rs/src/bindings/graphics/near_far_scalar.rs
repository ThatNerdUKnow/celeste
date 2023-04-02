use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type NearFarScalar;

    #[wasm_bindgen(constructor)]
    pub fn new(near: f64, nearValue: f64, far: f64, farValue: f64) -> NearFarScalar;
}
