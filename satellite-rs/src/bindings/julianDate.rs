use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type JulianDate;

    #[wasm_bindgen(static_method_of= JulianDate)]
    pub fn fromIso8601(iso8601String: String) -> JulianDate;

    #[wasm_bindgen(static_method_of=JulianDate)]
    pub fn toIso8601(date: &JulianDate)-> String;

    #[wasm_bindgen(static_method_of=JulianDate)]
    pub fn secondsDifference(left: &JulianDate, right: &JulianDate) -> f64;
}
