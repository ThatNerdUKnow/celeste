#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub(super) name: &'static str,
    pub(super) id: &'static str,
}
