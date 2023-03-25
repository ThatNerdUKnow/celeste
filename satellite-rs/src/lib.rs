pub mod error;
mod utils;
pub mod bindings;

use error::Sgp4Error;
use js_sys::Array;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};

use sgp4::parse_3les;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


