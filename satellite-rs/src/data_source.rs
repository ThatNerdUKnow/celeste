use std::collections::HashSet;

use log::{debug, error, info, trace};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod data_fetching;
mod inheritance;

use crate::{
    bindings::{
        data_source_clock::DataSourceClock, entity_cluster::EntityCluster,
        entity_collection::EntityCollection, event::Event, julian_date::JulianDate,
    },
    satellite::Satellite,
};

#[wasm_bindgen]
pub struct SatelliteDataSource {
    satellites: Option<HashSet<Satellite>>,
    changed_event: Event,
    clock: DataSourceClock,
    clustering: EntityCluster,
    entities: EntityCollection,
    error_event: Event,
    is_loading: bool,
    loading_event: Event,
    name: String,
    show: bool,
}

#[wasm_bindgen]
impl SatelliteDataSource {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SatelliteDataSource {
        info!("Creating Satellite data source");
        SatelliteDataSource {
            satellites: None,
            changed_event: Event::new(),
            clock: DataSourceClock::new(),
            clustering: EntityCluster::new(),
            entities: EntityCollection::new(),
            error_event: Event::new(),
            is_loading: true,
            loading_event: Event::new(),
            name: String::from("Celestrak"),
            show: true,
        }
    }

    #[wasm_bindgen]
    pub fn update(&self, time: JulianDate) -> bool {
        let date = JulianDate::toIso8601(&time);
        trace!("Update called");
        trace!("Current Clock: {}", date);
        console::time_with_label(&date);

        let ready = match &self.satellites {
            Some(satellites) => {
                for satellite in satellites {
                    match satellite.propogate(&time) {
                        Ok(prediction) => satellite.update_entity(prediction),
                        Err(e) => error!("{e}"),
                    }
                }
                true
            }
            None => false,
        };
        console::time_end_with_label(&date);

        ready
    }
}

impl Default for SatelliteDataSource {
    fn default() -> Self {
        SatelliteDataSource::new()
    }
}
