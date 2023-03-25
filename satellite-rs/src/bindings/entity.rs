use wasm_bindgen::prelude::*;

use super::cartesian3::Cartesian3;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Entity;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Entity;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &Entity, val: String);

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &Entity, val: Cartesian3);
}
