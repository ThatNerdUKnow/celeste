use wasm_bindgen::prelude::*;

use crate::bindings::julianDate::JulianDate;

#[wasm_bindgen(module = "cesium")]
extern "C" {

    pub type Event;

    pub type DataSourceClock;

    pub type EntityCluster;

    pub type EntityCollection;
}

#[wasm_bindgen]
pub struct SatelliteDataSource {
    //raw_entities: Vec<(sgp4::Elements, Entity)>,
    changedEvent: Event,
    clock: DataSourceClock,
    clustering: EntityCluster,
    entities: EntityCollection,
    errorEvent: Event,
    isLoading: bool,
    loadingEvent: Event,
    name: String,
    show: bool,
    constants: sgp4::Constants,
}

#[wasm_bindgen]
impl SatelliteDataSource {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SatelliteDataSource {
        todo!()
    }

    pub fn update(time: JulianDate) -> bool {
        true
    }
}
