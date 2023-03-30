use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum ExtrapolationType {
    NONE = 0,
    HOLD = 1,
    EXTRAPOLATE = 2,
}
