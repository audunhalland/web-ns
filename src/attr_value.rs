///
/// A typed attribute value.
///
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum AttributeValue {
    Empty,
    True,
    False,
    String(String),
    Multi(Vec<String>),
}
