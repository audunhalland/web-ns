///
/// The runtime type of an attribute.
///
#[derive(Clone)]
pub struct AttrType(pub u32);

pub mod flags {
    /// A boolean property: false if omitted, true if specified without value.
    pub const BOOL: u32 = 0x1;

    /// The explicit value "true":
    pub const TRUE: u32 = 0x2;

    /// The explicit value "false":
    pub const FALSE: u32 = 0x4;

    /// Any string:
    pub const STRING: u32 = 0x8;

    // The empty string
    pub const EMPTY_STRING: u32 = 0x10;

    // Any number:
    pub const NUMBER: u32 = 0x20;

    pub const COMMA_SEP: u32 = 0x100;
    pub const SPACE_SEP: u32 = 0x200;
    pub const COMMA_OR_SPACE_SEP: u32 = 0x400;
}

///
/// Type for an attribute value
///
pub enum PrimitiveType {
    Boolean,
    True,
    False,
    String,
    EmptyString,
    Number,
}

///
/// The way an attribute value may be quantified
///
#[derive(Clone)]
pub enum Quantifier {
    One,
    CommaSeparated,
    SpaceSeparated,
    CommaOrSpaceSeparated,
}

pub struct TypeFlags {}

pub fn string() -> AttrType {
    AttrType(flags::STRING)
}
