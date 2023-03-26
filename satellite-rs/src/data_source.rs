use log::{debug, error, info, trace};
use wasm_bindgen::prelude::*;

use crate::bindings::{
    data_source_clock::DataSourceClock, entity_cluster::EntityCluster,
    entity_collection::EntityCollection, event::Event, julian_date::JulianDate,
    satellite::Satellite,
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
        debug!("Update called");
        trace!("Current Clock: {}", JulianDate::toIso8601(&time));

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
        debug!("Loading Satellite Data");
        todo!()
    }

    #[wasm_bindgen(getter)]
    #[allow(non_snake_case)]
    pub fn changedEvent(&self) -> JsValue {
        trace!("changedEvent");
        self.changed_event.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn clock(&self) -> JsValue {
        trace!("clock");
        self.clock.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_clock(&mut self, clock: DataSourceClock) {
        trace!("set_clock");
        self.clock = clock
    }

    #[wasm_bindgen(getter)]
    pub fn clustering(&self) -> JsValue {
        trace!("clustering");
        self.clustering.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_clustering(&mut self, clustering: EntityCluster) {
        trace!("set_clustering");
        self.clustering = clustering
    }

    /// This will likely be slow
    #[wasm_bindgen(getter)]
    pub fn entities(&self) -> JsValue {
        trace!("entities");
        self.entities.clone()
    }

    #[wasm_bindgen(getter)]
    #[allow(non_snake_case)]
    pub fn errorEvent(&self) -> JsValue {
        trace!("errorEvent");
        self.error_event.clone()
    }

    #[wasm_bindgen(getter)]
    #[allow(non_snake_case)]
    pub fn isLoading(&self) -> bool {
        trace!("isLoading");
        self.is_loading
    }

    #[wasm_bindgen(getter)]
    #[allow(non_snake_case)]
    pub fn loadingEvent(&self) -> JsValue {
        trace!("loadingEvent");
        self.loading_event.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        trace!("name");
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn show(&self) -> bool {
        trace!("show");
        self.show
    }
}

impl Default for SatelliteDataSource {
    fn default() -> Self {
        SatelliteDataSource::new()
    }
}
