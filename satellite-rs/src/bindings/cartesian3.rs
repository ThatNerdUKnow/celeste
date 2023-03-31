use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::adapter::cartesian3::Cartesian3;

use super::{cartesian4::Cartesian4, ellipsoid::Ellipsoid, spherical::Spherical};

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(js_name = "Cartesian3")]
    pub type JSCartesian3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> JSCartesian3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &JSCartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &JSCartesian3, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &JSCartesian3, val: f64);

}

/// Static methods
#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="packedLength")]
    pub fn packed_length(this: &JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn abs(left: JSCartesian3, right: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn add(left: JSCartesian3, right: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="angleBetween")]
    pub fn angle_between(left: JSCartesian3, right: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn clamp(
        cartesian: JSCartesian3,
        min: JSCartesian3,
        max: JSCartesian3,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn clone(cartesian: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn cross(left: JSCartesian3, right: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn distance(left: JSCartesian3, right: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="divideByScalar")]
    pub fn divide_by_scalar(
        cartesian: JSCartesian3,
        scalar: f64,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="divideComponents")]
    pub fn divide_components(
        left: JSCartesian3,
        right: JSCartesian3,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn equals(left: Option<JSCartesian3>, right: Option<JSCartesian3>) -> bool;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="equalsEpsilon")]
    pub fn equals_epsilon(
        left: JSCartesian3,
        right: JSCartesian3,
        relative_epsilon: f64,
        absolute_epsilon: f64,
    ) -> bool;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromArray")]
    pub fn from_array(array: Array, starting_index: f64, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromCartesian4")]
    pub fn from_cartesian_4(cartesian: Cartesian4, result: Option<JSCartesian3>) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromDegrees")]
    pub fn from_degrees(
        longitude: f64,
        latitude: f64,
        height: Option<f64>,
        ellipsoid: Option<Ellipsoid>,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromDegreesArray")]
    pub fn from_degrees_array(
        coordinates: Array,
        ellipsoid: Option<Ellipsoid>,
        result: Option<Array>,
    ) -> Array;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromDegreesArrayHeights")]
    pub fn from_degrees_array_heights(
        coordinates: Array,
        ellipsoid: Option<Ellipsoid>,
        result: Option<Array>,
    ) -> Array;

    #[wasm_bindgen(static_method_of = JSCartesian3)]
    pub fn fromElements(x: f64, y: f64, z: f64) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromRadians")]
    pub fn from_radians(
        longitude: f64,
        latitude: f64,
        height: Option<f64>,
        ellipsoid: Option<Ellipsoid>,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromRadiansArray")]
    pub fn from_radians_array(
        coordinates: Array,
        ellipsoid: Option<Ellipsoid>,
        result: Array,
    ) -> Array;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromRadiansArrayHeights")]
    pub fn from_radians_array_heights(
        coordinates: Array,
        ellipsoid: Option<Ellipsoid>,
        result: Array,
    ) -> Array;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="fromSpherical")]
    pub fn from_spherical(spherical: Spherical, result: Option<JSCartesian3>) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn lerp(
        start: JSCartesian3,
        end: JSCartesian3,
        t: f64,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn magnitude(cartesian: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="magnitudeSquared")]
    pub fn magnitude_squared(cartesian: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="maximumByComponent")]
    pub fn maximum_by_component(
        first: JSCartesian3,
        second: JSCartesian3,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="maximumComponent")]
    pub fn maximum_component(cartesian: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn midpoint(left: JSCartesian3, right: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="minimumByComponent")]
    pub fn minimum_by_component(
        first: JSCartesian3,
        second: JSCartesian3,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="minimumComponent")]
    pub fn minumum_component(cartesian: JSCartesian3) -> f64;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="mostOrthogonalAxis")]
    pub fn most_orthogonal_axis(cartesian: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="multiplyByScalar")]
    pub fn multiply_by_scalar(
        cartesian: JSCartesian3,
        scalar: f64,
        result: JSCartesian3,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn negate(cartesian: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn normalize(cartesian: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn pack(value: JSCartesian3, array: Array, startingIndex: Option<f64>) -> Array;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn project_vector(a: JSCartesian3, b: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn subtract(left: JSCartesian3, right: JSCartesian3, result: JSCartesian3) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3)]
    pub fn unpack(
        array: Array,
        startingIndex: Option<f64>,
        result: Option<JSCartesian3>,
    ) -> JSCartesian3;

    #[wasm_bindgen(static_method_of=JSCartesian3,js_name="unpackArray")]
    pub fn unpack_array(
        array: Array,
        startingIndex: Option<f64>,
        result: Option<JSCartesian3>,
    ) -> JSCartesian3;

}
