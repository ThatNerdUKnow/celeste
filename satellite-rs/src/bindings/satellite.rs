use chrono::NaiveDateTime;
use error_stack::{IntoReport, ResultExt};
use sgp4::{Elements, Prediction};
use std::collections::HashSet;

use crate::error::{Error, WrapSgp4Error};

use super::{cartesian3::Cartesian3, entity::Entity, julian_date::JulianDate};

pub struct Satellite {
    entity: Entity,
    elements: Elements,
    constants: sgp4::Constants,
    categories: HashSet<String>,
}

impl Satellite {
    pub fn new(tle: &str, categories: HashSet<String>) -> Result<Satellite, sgp4::Error> {
        let ent = Entity::new();

        let lines: Vec<&str> = tle.split('\n').collect();

        assert_eq!(lines.len(), 3);

        let elements = Elements::from_tle(
            Some(lines[0].to_owned()),
            lines[1].as_bytes(),
            lines[2].as_bytes(),
        )?;

        let constants = sgp4::Constants::from_elements(&elements)?;

        Ok(Satellite {
            entity: ent,
            elements,
            constants,
            categories,
        })
    }

    /// Propogate satellite's position given [`JulianDate`] from cesium
    pub fn propogate(&self, date: &JulianDate) -> error_stack::Result<Prediction, Error> {
        let iso8601 = JulianDate::toIso8601(date);

        let date = NaiveDateTime::parse_from_str(&iso8601, "%Y-%m-%dT%H:%M:%SZ")
            .into_report()
            .change_context(Error::Propogate)?;

        let minutes = self
            .elements
            .minutes_since_epoch(&date)
            .to_sgp4_report()
            .change_context(Error::Propogate)?;

        self.constants
            .propagate(minutes)
            .to_sgp4_report()
            .change_context(Error::Propogate)
    }

    pub fn update_entity(&self, prediction: Prediction) {
        let [x, y, z] = prediction.position;

        let coords = Cartesian3::fromElements(x, y, z);

        self.entity.set_position(coords);
    }
}
