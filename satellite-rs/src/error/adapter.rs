use error_stack::Report;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
#[wasm_bindgen]
#[error("{0}")]
pub struct ErrorStackAdapter(Report<super::Error>);

impl From<Report<super::Error>> for ErrorStackAdapter {
    fn from(value: Report<super::Error>) -> Self {
        ErrorStackAdapter(value)
    }
}
