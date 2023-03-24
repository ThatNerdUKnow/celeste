use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Elements(sgp4::Elements);

impl Elements {
    pub fn new(elements: sgp4::Elements) -> Self {
        Elements(elements)
    }
}