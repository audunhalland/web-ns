///
/// The runtime type of an attribute.
///
pub struct AttrType {
    pub quantifier: Quantifier,
    pub primitives: &'static [PrimitiveType],
}

///
/// Type for an attribute value
///
pub enum PrimitiveType {
    // A boolean property: false if omitted, true if specified without value.
    Boolean,
    // The explicit value "true":
    True,
    // The explicit value "false":
    False,
    // Any string:
    String,
    // The empty string
    EmptyString,
    // Any number:
    Number,
}

///
/// The way an attribute value may be quantified
///
pub enum Quantifier {
    One,
    CommaSeparated,
    SpaceSeparated,
    CommaOrSpaceSeparated,
}
