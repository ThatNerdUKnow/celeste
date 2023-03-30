use wasm_bindgen::prelude::*;

use crate::bindings::position_property::reference_frame::ReferenceFrame;

use super::{cartesian3::Cartesian3, julian_date::JulianDate, property::Property};

pub mod reference_frame;
pub mod sampled_position_property;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=Property)]
    pub type PositionProperty;

    #[wasm_bindgen(method, getter)]
    pub fn referenceFrame(this: &PositionProperty) -> ReferenceFrame;

    #[wasm_bindgen(method, setter)]
    pub fn set_referenceFrame(this: &PositionProperty, val: ReferenceFrame);

    #[wasm_bindgen(method)]
    pub fn getValueInReferenceFrame(
        this: &PositionProperty,
        time: &JulianDate,
        referenceFrame: ReferenceFrame,
        result: &Cartesian3,
    ) -> Option<Cartesian3>;
}
