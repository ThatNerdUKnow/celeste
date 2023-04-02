use std::collections::BTreeSet;

use chrono::NaiveDateTime;
use error_stack::{IntoReport, ResultExt};
use log::{error, info, trace};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod data_fetching;
mod inheritance;

use crate::{
    bindings::{
        clock::{clock_step::ClockStep, data_source_clock::DataSourceClock},
        entity_cluster::EntityCluster,
        entity_collection::EntityCollection,
        event::Event,
        julian_date::JulianDate,
    },
    error::{adapter::ErrorStackAdapter, Error},
    satellite::Satellite,
};

#[wasm_bindgen]
pub struct SatelliteDataSource {
    satellites: Option<BTreeSet<Satellite>>,
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
    pub fn new(clock: DataSourceClock) -> SatelliteDataSource {
        info!("Creating Satellite data source");

        SatelliteDataSource {
            satellites: None,
            changed_event: Event::new(),
            clock,
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
    pub fn update(&self, orig_date: JulianDate) -> Result<bool, ErrorStackAdapter> {
        let iso8601 = JulianDate::toIso8601(&orig_date);
        trace!("Update called");
        trace!("Current Clock: {}", iso8601);
        console::time_with_label(&iso8601);

        let date = NaiveDateTime::parse_from_str(&iso8601, "%+")
            .into_report()
            .map_err(|e| {
                error!("{e}");
                e
            })
            .change_context(Error::Propogate)?;

        let ready = match &self.satellites {
            Some(satellites) => {
                for satellite in satellites {
                    match satellite.propogate(&date) {
                        Ok(prediction) => satellite.update_entity(&orig_date, prediction),
                        Err(e) => error!("{e}"),
                    }
                }
                true
            }
            None => false,
        };
        console::time_end_with_label(&iso8601);

        Ok(ready)
    }
}
