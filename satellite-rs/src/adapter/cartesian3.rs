use std::{clone, fmt::Display};

use log::error;
use wasm_bindgen::prelude::*;

use crate::bindings::cartesian3::JSCartesian3;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cartesian3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Cartesian3 {
    #[wasm_bindgen]
    pub fn clone(&self, result: Option<JSCartesian3>) -> JSCartesian3 {
        match result {
            Some(result) => {
                result.set_x(self.x);
                result.set_y(self.y);
                result.set_z(self.z);
                result
            }
            None => Clone::clone(self).into(),
        }
    }

    #[wasm_bindgen]
    pub fn equals(&self, right: JSCartesian3) -> bool {
        self.eq(&right)
    }

    #[wasm_bindgen(js_name = "equalsEpsilon")]
    pub fn equals_epsilon(
        &self,
        _right: JSCartesian3,
        _relative_epsilon: f64,
        _absolute_epsilon: f64,
    ) -> bool {
        error!("equalsEpsilon Called");
        todo!()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_compat(&self) -> String {
        format!("{self}")
    }
}

impl From<Cartesian3> for JSCartesian3 {
    fn from(value: Cartesian3) -> Self {
        JSCartesian3::fromElements(value.x, value.y, value.z)
    }
}

impl From<JSCartesian3> for Cartesian3 {
    fn from(value: JSCartesian3) -> Self {
        Cartesian3 {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }
}

impl PartialEq<JSCartesian3> for Cartesian3 {
    fn eq(&self, other: &JSCartesian3) -> bool {
        let x = self.x == other.x();
        let y = self.y == other.y();
        let z = self.z == other.z();
        x && y && z
    }
}

impl Display for Cartesian3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
