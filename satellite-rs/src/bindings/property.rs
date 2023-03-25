use wasm_bindgen::{JsValue, convert::IntoWasmAbi};

pub trait Property<> {
    fn equals(other: impl Property) -> bool;
}
