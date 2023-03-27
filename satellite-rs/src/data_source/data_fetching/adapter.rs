use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use sgp4::Elements;

use crate::data::group::Group;
use crate::satellite::Satellite;

pub struct ElementsAdapter(Elements);

impl Hash for ElementsAdapter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.norad_id.hash(state);
    }
}

impl PartialEq for ElementsAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0.norad_id == other.0.norad_id
    }
}

impl Into<Elements> for ElementsAdapter {
    fn into(self) -> Elements {
        self.0
    }
}

impl Eq for ElementsAdapter {}

impl From<Elements> for ElementsAdapter {
    fn from(value: Elements) -> Self {
        ElementsAdapter(value)
    }
}

impl Debug for ElementsAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ElementsAdapter")
            .field(&self.0.object_name)
            .finish()
    }
}

impl From<(ElementsAdapter, BTreeSet<&'static Group>)> for Satellite {
    fn from((elements, categories): (ElementsAdapter, BTreeSet<&'static Group>)) -> Self {
        Satellite::new(elements.into(), categories).unwrap()
    }
}

impl AsRef<Elements> for ElementsAdapter {
    fn as_ref(&self) -> &Elements {
        &self.0
    }
}

impl PartialOrd for ElementsAdapter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.norad_id.partial_cmp(&other.0.norad_id)
    }
}

impl Ord for ElementsAdapter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.norad_id.cmp(&other.0.norad_id)
    }
}
