use chrono::NaiveDateTime;
use error_stack::{IntoReport, ResultExt};
use log::{debug, error, trace};
use result_inspect::ResultInspectErr;
use sgp4::{Elements, Prediction};
use std::hash::Hash;
use std::{collections::BTreeSet, convert::TryInto};

use crate::{
    bindings::{cartesian3::Cartesian3, entity::Entity, julian_date::JulianDate},
    data::group::Group,
    data_source::data_fetching::adapter::ElementsAdapter,
    error::{Error, WrapSgp4Error},
};

pub struct Satellite {
    entity: Entity,
    elements: ElementsAdapter,
    constants: sgp4::Constants,
    categories: BTreeSet<&'static Group>,
}

impl Hash for Satellite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        //self.entity.hash(state);
        self.elements.hash(state);
        //self.constants.hash(state);
        //self.categories.hash(state);
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl Eq for Satellite {}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.elements
            .as_ref()
            .norad_id
            .partial_cmp(&other.elements.as_ref().norad_id)
    }
}

impl Ord for Satellite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.elements
            .as_ref()
            .norad_id
            .cmp(&other.elements.as_ref().norad_id)
    }
}

impl Satellite {
    pub fn new(
        elements: Elements,
        categories: BTreeSet<&'static Group>,
    ) -> error_stack::Result<Satellite, Error> {
        trace!("Creating new Satellite Data Source");
        let ent = Entity::new();

        let constants = sgp4::Constants::from_elements(&elements)
            .to_sgp4_report()
            .attach_printable("Creating constants for satellite")
            .change_context(Error::GetSats)?;

        Ok(Satellite {
            entity: ent,
            elements: elements.into(),
            constants,
            categories,
        })
    }

    /// Propogate satellite's position given [`JulianDate`] from cesium
    pub fn propogate(&self, date: &NaiveDateTime) -> error_stack::Result<Prediction, Error> {
        //let iso8601 = JulianDate::toIso8601(date);

        //let date = NaiveDateTime::parse_from_str(&iso8601, "%+")
        //    .into_report()
        //    .inspect_err(|e| error!("{e}"))
        //    .change_context(Error::Propogate)?;

        let minutes = self
            .elements
            .as_ref()
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

    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}
