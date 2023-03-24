use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sgp4Error(sgp4::Error);

impl From<sgp4::Error> for Sgp4Error {
    fn from(value: sgp4::Error) -> Self {
        Sgp4Error(value)
    }
}
