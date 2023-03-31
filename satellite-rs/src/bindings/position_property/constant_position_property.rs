use wasm_bindgen::prelude::wasm_bindgen;

use crate::bindings::{cartesian3::JSCartesian3, position_property::PositionProperty};

use super::reference_frame::ReferenceFrame;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=PositionProperty)]
    pub type ConstantPositionProperty;

    #[wasm_bindgen(js_namespace=ConstantPositionProperty,js_name="ConstantPositionProperty")]
    static CONSTANT_POSITION_PROPERTY: ConstantPositionProperty;

    #[wasm_bindgen(method)]
    pub fn setValue(
        this: &ConstantPositionProperty,
        value: JSCartesian3,
        referenceFrame: ReferenceFrame,
    );
}
