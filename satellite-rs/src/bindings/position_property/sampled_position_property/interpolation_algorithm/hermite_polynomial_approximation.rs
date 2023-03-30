use wasm_bindgen::prelude::wasm_bindgen;

use super::InterpolationAlgorithm;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(extends=InterpolationAlgorithm)]
    pub type HermitePolynomialApproximation;

    #[wasm_bindgen(js_namespace=HermitePolynomialApproximation,js_name="HermitePolynomialApproximation")]
    static HERMITE_POLYNOMIAL_APPROXIMATION: HermitePolynomialApproximation;
}
