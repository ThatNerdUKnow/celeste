pub mod error;
pub mod satellites;
mod utils;

use error::Sgp4Error;
use js_sys::Array;
use satellites::Elements;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn parse_sat(tles: &str) -> Result<Elements, Sgp4Error> {
    let elements: Elements = parse_3les(tles)?
        .into_iter()
        .map(Elements::new).next()
        .expect("Couldn't parse satellite");

    Ok(elements)
}
