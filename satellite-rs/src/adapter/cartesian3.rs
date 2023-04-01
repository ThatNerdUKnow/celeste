use std::{clone, fmt::Display};

use log::error;
use wasm_bindgen::prelude::*;

use crate::bindings::cartesian3::Cartesian3;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cartesian3Adapter {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Cartesian3Adapter {
    #[wasm_bindgen]
    pub fn clone(&self, result: Option<Cartesian3>) -> Cartesian3 {
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
    pub fn equals(&self, right: Cartesian3) -> bool {
        self.eq(&right)
    }

    #[wasm_bindgen(js_name = "equalsEpsilon")]
    pub fn equals_epsilon(
        &self,
        _right: Cartesian3,
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

impl From<Cartesian3Adapter> for Cartesian3 {
    fn from(value: Cartesian3Adapter) -> Self {
        Cartesian3::fromElements(value.x, value.y, value.z)
    }
}

impl From<Cartesian3> for Cartesian3Adapter {
    fn from(value: Cartesian3) -> Self {
        Cartesian3Adapter {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }
}

impl PartialEq<Cartesian3> for Cartesian3Adapter {
    fn eq(&self, other: &Cartesian3) -> bool {
        let x = self.x == other.x();
        let y = self.y == other.y();
        let z = self.z == other.z();
        x && y && z
    }
}

impl Display for Cartesian3Adapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
