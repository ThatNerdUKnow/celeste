use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::bindings::{
    cartesian3::JSCartesian3, julian_date::JulianDate, position_property::PositionProperty,
};

use self::{
    extrapolation_type::ExtrapolationType, interpolation_algorithm::InterpolationAlgorithm,
};

use super::reference_frame::ReferenceFrame;

mod extrapolation_type;
mod interpolation_algorithm;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(extends=PositionProperty)]
    pub type SampledPositionProperty;

    #[wasm_bindgen(constructor)]
    pub fn new(referenceFrame: ReferenceFrame, numberOfDerivatives: f64)
        -> SampledPositionProperty;

    #[wasm_bindgen(method, getter)]
    pub fn backwardExtrapolationDuration(this: &SampledPositionProperty) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_backwardExtrapolationDuration(this: &SampledPositionProperty, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn backwardExtrapolationType(this: &SampledPositionProperty) -> ExtrapolationType;

    #[wasm_bindgen(method, setter)]
    pub fn set_backwardExtrapolationType(this: &SampledPositionProperty, val: ExtrapolationType);

    #[wasm_bindgen(method, getter)]
    pub fn forwardExtrapolationDuration(this: &SampledPositionProperty) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_forwardExtrapolationDuration(this: &SampledPositionProperty, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn forwardExtrapolationType(this: &SampledPositionProperty) -> ExtrapolationType;

    #[wasm_bindgen(method, setter)]
    pub fn set_forwardExtrapolationType(this: &SampledPositionProperty, val: ExtrapolationType);

    #[wasm_bindgen(method, getter)]
    pub fn interpolationAlgorithm(this: &SampledPositionProperty) -> InterpolationAlgorithm;

    #[wasm_bindgen(method, setter)]
    pub fn set_interpolationAlgorithm(this: &SampledPositionProperty, val: &InterpolationAlgorithm);

    #[wasm_bindgen(method, getter)]
    pub fn interpolationDegree(this: &SampledPositionProperty) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn numberOfDerivatives(this: &SampledPositionProperty) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_numberOfDerivatives(this: &SampledPositionProperty, val: f64);

    #[wasm_bindgen(method)]
    pub fn addSample(
        this: &SampledPositionProperty,
        time: &JulianDate,
        position: JSCartesian3,
        derivatives: Option<Array>,
    );
}
