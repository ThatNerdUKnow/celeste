use wasm_bindgen::prelude::*;

use crate::bindings::graphics::color::Color;

use super::near_far_scalar::NearFarScalar;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type PointGraphics;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PointGraphics;

    #[wasm_bindgen(method, setter)]
    pub fn set_color(this: &PointGraphics, val: &Color);

    #[wasm_bindgen(method, setter, js_name = "translucencyByDistance")]
    pub fn set_translucency_by_distance(this: &PointGraphics, prop: NearFarScalar);
}
