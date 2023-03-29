use wasm_bindgen::prelude::*;

use crate::bindings::graphics::color::Color;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type PointGraphics;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PointGraphics;

    #[wasm_bindgen(method, setter)]
    pub fn set_color(this: &PointGraphics, val: &Color);
}
