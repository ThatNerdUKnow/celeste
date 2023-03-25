use log::error;
use wasm_bindgen::prelude::*;

use crate::bindings::julianDate::JulianDate;

use super::{
    dataSourceClock::DataSourceClock, entity_cluster::EntityCluster,
    entity_collection::EntityCollection, event::Event, satellite::Satellite,
};

#[wasm_bindgen]
pub struct SatelliteDataSource {
    satellites: Option<Vec<Satellite>>,
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
        match &self.satellites {
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
        }
    }

    #[wasm_bindgen]
    pub async fn load_data(&mut self) {
        todo!()
    }
}

impl Default for SatelliteDataSource {
    fn default() -> Self {
        SatelliteDataSource::new()
    }
}
