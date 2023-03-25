use log::error;
use wasm_bindgen::prelude::*;

use crate::bindings::julianDate::JulianDate;

use super::satellite::Satellite;

#[wasm_bindgen(module = "cesium")]
extern "C" {

    pub type Event;

    pub type DataSourceClock;

    pub type EntityCluster;

    pub type EntityCollection;
}

#[wasm_bindgen]
pub struct SatelliteDataSource {
    satellites: Vec<Satellite>,
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

    #[wasm_bindgen(method)]
    pub fn update(&self, time: JulianDate) -> bool {
        for satellite in &self.satellites {
            match satellite.propogate(&time){
                Ok(prediction) => todo!(),
                Err(e) => error!("{e}")
            }
        }
        true
    }
}
