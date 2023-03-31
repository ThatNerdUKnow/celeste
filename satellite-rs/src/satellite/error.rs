use chrono::NaiveDateTime;
use thiserror::Error;

use crate::data_source::data_fetching::adapter::ElementsAdapter;

#[derive(Error, Debug)]
pub enum SatelliteError {
    #[error("Couldn't create satellite")]
    CreateSatellite,
    #[error("Couldn't propogate satellite {0} at {1}")]
    Propogate(String, NaiveDateTime),
}
