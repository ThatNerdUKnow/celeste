pub mod bindings;
mod data_source;
pub mod error;
mod utils;
pub mod data;

use log::info;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    //#[cfg(feature = "console_error_panic_hook")]
    //panic::set_hook(Box::new(console_error_panic_hook::hook));

    //set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    info!("Logging Initalized")
}
