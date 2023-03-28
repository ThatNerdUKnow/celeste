use std::{collections::{HashSet, BTreeSet}, str::FromStr};

use chrono::NaiveDateTime;
use error_stack::{IntoReport, ResultExt};
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
    pub fn update(&self, date: JulianDate) -> Result<bool, ErrorStackAdapter> {
        let iso8601 = JulianDate::toIso8601(&date);
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
                        Ok(prediction) => satellite.update_entity(prediction),
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

impl Default for SatelliteDataSource {
    fn default() -> Self {
        SatelliteDataSource::new()
    }
}
