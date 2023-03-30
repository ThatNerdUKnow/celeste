use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum ReferenceFrame {
    FIXED = 0,
    INERTIAL = 1,
}
