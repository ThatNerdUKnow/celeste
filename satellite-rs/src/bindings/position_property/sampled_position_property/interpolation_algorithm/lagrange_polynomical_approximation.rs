use wasm_bindgen::prelude::wasm_bindgen;

use crate::bindings::position_property::sampled_position_property::interpolation_algorithm::InterpolationAlgorithm;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(extends=InterpolationAlgorithm)]
    pub type LagrangePolynomialApproximation;

    #[wasm_bindgen(js_namespace=InterpolationAlgorithm,js_name="LagrangePolynomialApproximation")]
    static LAGRANGE_POLYNOMIAL_APPROXIMATION: LagrangePolynomialApproximation;
}
