#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Group {
    pub(super) name: &'static str,
    pub(super) id: &'static str,
}

impl Group {
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
