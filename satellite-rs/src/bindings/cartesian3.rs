use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type Cartesian3;

    #[wasm_bindgen(static_method_of = Cartesian3)]
    pub fn fromElements(x: f64, y: f64, z: f64) -> Cartesian3;
}
