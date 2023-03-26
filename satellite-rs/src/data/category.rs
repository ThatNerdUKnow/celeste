use std::collections::BTreeSet;

use super::group::Group;

#[derive(Hash, PartialEq, Eq)]
pub struct Category {
    pub(super) name: &'static str,
    pub(super) groups: BTreeSet<Group>,
}
