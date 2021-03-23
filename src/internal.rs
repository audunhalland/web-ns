#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InternalAttr {
    pub attribute: &'static str,
    pub property: &'static str,
}
