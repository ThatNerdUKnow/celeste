use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use log::trace;
use sgp4::Elements;

pub struct ElementsAdapter(Elements);

impl Hash for ElementsAdapter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.norad_id.hash(state);
        self.0.object_name.hash(state);
        self.0.international_designator.hash(state)
    }
}

impl PartialEq for ElementsAdapter {
    fn eq(&self, other: &Self) -> bool {
        let mut matches = false;
        matches = matches || self.0.norad_id == other.0.norad_id;
        matches = matches && self.0.object_name == other.0.object_name;
        matches = matches && self.0.international_designator == other.0.international_designator;

        matches
    }
}

impl Eq for ElementsAdapter {}

impl From<Elements> for ElementsAdapter {
    fn from(value: Elements) -> Self {
        trace!("Wrapping Element {:#?}", value.object_name);
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
