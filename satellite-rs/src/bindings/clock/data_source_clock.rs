use wasm_bindgen::prelude::wasm_bindgen;

use crate::bindings::{
    clock::{clock_range::ClockRange, clock_step::ClockStep},
    event::Event,
    julian_date::JulianDate,
};

use super::Clock;

#[wasm_bindgen(module = "cesium")]
extern "C" {
    pub type DataSourceClock;

    #[wasm_bindgen(constructor)]
    pub fn new() -> DataSourceClock;

    #[wasm_bindgen(method, getter)]
    pub fn clockRange(this: &DataSourceClock) -> ClockRange;

    #[wasm_bindgen(method, setter)]
    pub fn set_clockRange(this: &DataSourceClock, val: ClockRange);

    #[wasm_bindgen(method, getter)]
    pub fn clockStep(this: &DataSourceClock) -> ClockStep;

    #[wasm_bindgen(method, setter)]
    pub fn set_clockStep(this: &DataSourceClock, val: ClockStep);

    #[wasm_bindgen(method, getter)]
    pub fn currentTime(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter)]
    pub fn set_currentTime(this: &DataSourceClock, val: &JulianDate);

    #[wasm_bindgen(method, getter)]
    pub fn definitionChanged(this: &DataSourceClock) -> Event;

    #[wasm_bindgen(method, getter)]
    pub fn multiplier(this: &DataSourceClock) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_multiplier(this: &DataSourceClock, val: f64);

    #[wasm_bindgen(method, getter)]
    pub fn startTime(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter)]
    pub fn set_startTime(this: &DataSourceClock, val: &JulianDate);

    #[wasm_bindgen(method, getter)]
    pub fn stopTime(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter)]
    pub fn set_stopTime(this: &DataSourceClock, val: &JulianDate);

    #[wasm_bindgen(method)]
    pub fn clone(result: &DataSourceClock) -> DataSourceClock;

    #[wasm_bindgen(method)]
    pub fn equals(other: &DataSourceClock) -> bool;

    #[wasm_bindgen(method)]
    pub fn getValue(this: &DataSourceClock) -> Clock;

    #[wasm_bindgen(method)]
    pub fn merge(source: &DataSourceClock);
}
