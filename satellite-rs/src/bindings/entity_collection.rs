use wasm_bindgen::prelude::wasm_bindgen;

use super::entity::Entity;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type EntityCollection;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EntityCollection;

    #[wasm_bindgen(method)]
    fn add(this: &EntityCollection, entity: &Entity) -> Entity;

    #[wasm_bindgen(method)]
    fn contains(this: &EntityCollection, entity: &Entity) -> bool;

}
