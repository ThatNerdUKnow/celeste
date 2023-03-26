pub mod bindings;
pub mod data;
mod data_source;
pub mod error;
mod satellite;
mod utils;

use std::panic;

use log::info;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    info!("Logging Initalized")
}
