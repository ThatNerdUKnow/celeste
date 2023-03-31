use std::fmt::Display;

use error_stack::IntoReport;
use log::error;
use thiserror::Error;
use wasm_bindgen::prelude::*;

pub mod adapter;

#[wasm_bindgen]
#[derive(Error, Debug)]
#[error("{0:?}")]
pub struct Sgp4Error(sgp4::Error);

impl From<sgp4::Error> for Sgp4Error {
    fn from(value: sgp4::Error) -> Self {
        Sgp4Error(value)
    }
}

pub trait WrapSgp4Error<T> {
    fn to_sgp4_report(self) -> error_stack::Result<T, Sgp4Error>;
}

impl<T> WrapSgp4Error<T> for Result<T, sgp4::Error> {
    fn to_sgp4_report(self) -> error_stack::Result<T, Sgp4Error> {
        self.map_err(|e| {
            let binding: Sgp4Error = e.into();
            error!("{binding:#?}");
            binding
        })
        .into_report()
    }
}

#[derive(Error, Debug)]
#[wasm_bindgen]
pub enum Error {
    #[error("Couldn't get satellite data")]
    GetSats,
    #[error("Couldn't propogate satellite")]
    Propogate,
}
