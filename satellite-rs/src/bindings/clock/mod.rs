pub mod data_source_clock;
pub mod clock_range;
pub mod clock_step;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Clock;
}