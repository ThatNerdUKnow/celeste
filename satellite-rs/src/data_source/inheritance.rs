use log::trace;
use wasm_bindgen::prelude::*;

use crate::bindings::{clock::data_source_clock::DataSourceClock, entity_cluster::EntityCluster};

use super::SatelliteDataSource;

#[wasm_bindgen]
impl SatelliteDataSource {
    #[wasm_bindgen(getter)]
    #[allow(non_snake_case)]
    pub fn changedEvent(&self) -> JsValue {
        trace!("changedEvent");
        self.changed_event.clone()
    }

 /*   #[wasm_bindgen(getter)]
    pub fn clock(&self) -> DataSourceClock {
        trace!("clock");
        self.clock.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_clock(&mut self, clock: DataSourceClock) {
        trace!("set_clock");
        self.clock = clock
    }*/

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
