use chrono::NaiveDateTime;
use error_stack::{IntoReport, ResultExt};
use log::{debug, error, trace};
use result_inspect::ResultInspectErr;
use sgp4::{Elements, Prediction};
use std::hash::Hash;
use std::{collections::BTreeSet, convert::TryInto};

use crate::bindings::entity::EntityAdapter;
use crate::bindings::graphics::point_graphics::PointGraphics;
use crate::bindings::position_property::reference_frame::ReferenceFrame;
use crate::bindings::position_property::sampled_position_property::SampledPositionProperty;
use crate::{
    bindings::{cartesian3::Cartesian3, entity::Entity, julian_date::JulianDate},
    data::group::Group,
    data_source::data_fetching::adapter::ElementsAdapter,
    error::{Error, WrapSgp4Error},
};

use self::error::SatelliteError;

pub mod error;

pub struct Satellite {
    entity: EntityAdapter<SampledPositionProperty>,
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
    ) -> error_stack::Result<Satellite, SatelliteError> {
        trace!("Creating new Satellite Data Source");

        let elements_adapter: ElementsAdapter = elements.into();
        let ent = Entity::new();
        //let position = Cartesian3::new();
        let point = PointGraphics::new();
        //let color = Color::new(1.0, 1.0, 1.0, 1.0);

        //point.set_color(&color);

        //ent.set_position(position);
        ent.set_point(point);

        let position_prop = SampledPositionProperty::new(ReferenceFrame::INERTIAL, 0.0);

        ent.set_position(position_prop.as_ref());

        let entity_adapter: EntityAdapter<SampledPositionProperty> =
            EntityAdapter::new(ent, position_prop);

        let constants = sgp4::Constants::from_elements(elements_adapter.as_ref())
            .to_sgp4_report()
            .attach_printable("Creating constants for satellite")
            .attach_printable(format!("{elements_adapter:#?}"))
            .change_context(SatelliteError::CreateSatellite)?;

        Ok(Satellite {
            entity: entity_adapter,
            elements: elements_adapter,
            constants,
            categories,
        })
    }

    /// Propogate satellite's position given [`JulianDate`] from cesium
    pub fn propogate(
        &self,
        date: &NaiveDateTime,
    ) -> error_stack::Result<Prediction, SatelliteError> {
        //let iso8601 = JulianDate::toIso8601(date);

        //let date = NaiveDateTime::parse_from_str(&iso8601, "%+")
        //    .into_report()
        //    .inspect_err(|e| error!("{e}"))
        //    .change_context(Error::Propogate)?;

        let minutes = self
            .elements
            .as_ref()
            .minutes_since_epoch(date)
            .to_sgp4_report()
            .change_context(SatelliteError::Propogate(
                format!("{:#?}", self.elements),
                *date,
            ))?;

        self.constants
            .propagate(minutes)
            .to_sgp4_report()
            .change_context(SatelliteError::Propogate(
                format!("{:#?}", self.elements),
                *date,
            ))
    }

    pub fn update_entity(&self, date: &JulianDate, prediction: Prediction) {
        let [x, y, z]: [f64; 3] = prediction
            .position
            .iter()
            .map(|km| km * 1000.0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let coords = Cartesian3::fromElements(x, y, z);

        //self.entity.set_position(coords);
        let sampled_position: &SampledPositionProperty = self.entity.as_ref();

        sampled_position.addSample(date, coords, None);
    }

    pub fn entity(&self) -> &Entity {
        &self.entity.as_ref()
    }
}
