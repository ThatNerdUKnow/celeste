use js_sys::{Array, Number};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod lagrange_polynomical_approximation;
pub mod linear_approximation;
pub mod hermite_polynomial_approximation;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type InterpolationAlgorithm;

    #[wasm_bindgen(method)]
    pub fn getRequiredDataPoints(this: &InterpolationAlgorithm, degree: f64) -> f64;

    #[wasm_bindgen]
    pub fn interpolate(
        this: &InterpolationAlgorithm,
        x: f64,
        xTable: Array,
        yTable: Array,
        yStride: f64,
        inputOrder: f64,
        outputOrder: f64,
        result: Array,
    ) -> Array;

    #[wasm_bindgen(method)]
    pub fn interpolateOrderZero(
        this: &InterpolationAlgorithm,
        x: f64,
        xTable: Array,
        yTable: Array,
        yStride: f64,
        result: Array,
    ) -> Array;
}
