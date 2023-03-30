use wasm_bindgen::prelude::*;

use crate::bindings::position_property::PositionProperty;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(extends=PositionProperty)]
    pub type SampledPositionProperty;

    #[wasm_bindgen(method, getter)]
    pub fn backwardExtrapolationDuration(this: &SampledPositionProperty) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_backwardExtrapolationDuration(this: &SampledPositionProperty, val: f64);
}
