use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Event;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Event;
}
