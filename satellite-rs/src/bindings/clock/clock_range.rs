use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum ClockRange {
    UNBOUNDED = 0,
    CLAMPED = 1,
    LOOP_STOP = 2,
}
