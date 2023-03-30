use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum ClockStep {
    TICK_DEPENDENT = 0,
    SYSTEM_CLOCK_MULTIPLIER = 1,
    SYSTEM_CLOCK = 2,
}
