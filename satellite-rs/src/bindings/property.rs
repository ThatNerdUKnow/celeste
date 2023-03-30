use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::{event::Event, julian_date::JulianDate};

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Property;

    #[wasm_bindgen(method, getter)]
    pub fn definitionChanged(this: &Property) -> Event;

    #[wasm_bindgen(method, getter)]
    pub fn isConstant(this: &Property) -> bool;

    #[wasm_bindgen(method)]
    pub fn equals(other: &Property) -> bool;

    #[wasm_bindgen(method)]
    pub fn getValue(time: &JulianDate, other: &Property) -> JsValue;
}
