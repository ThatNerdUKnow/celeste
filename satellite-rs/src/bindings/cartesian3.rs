use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Cartesian3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Cartesian3;

    #[wasm_bindgen(static_method_of = Cartesian3)]
    pub fn fromElements(x: f64, y: f64, z: f64) -> Cartesian3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Cartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Cartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Cartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Cartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Cartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Cartesian3, val: f64);

}
