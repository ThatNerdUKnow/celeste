use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(js_name = "Cartesian3")]
    pub type JSCartesian3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> JSCartesian3;

    #[wasm_bindgen(static_method_of = JSCartesian3)]
    pub fn fromElements(x: f64, y: f64, z: f64) -> JSCartesian3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &JSCartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &JSCartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &JSCartesian3, val: f64);

}
