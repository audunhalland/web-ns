pub enum AttrFormat {
    String,

    // The property is boolean. The default value of this property is false, so it can be omitted
    Bool,

    BoolOrEmptyString,

    // The property is a boolean. The default value of this property is something other than false, so false must persist. The value can hold a string (as is the case with ariaChecked and its 'mixed' value)
    OverloadedBool,

    // The property is a boolean. The default value of this property is something other than false, so false must persist. The value can hold a string (as is the case with ariaChecked and its 'mixed' value)
    BooleanIsh,

    // CommaSeparated,
    // SpaceSeparated,

    Enum,

    // The property is number. These values can sometimes hold a string
    Number,
}

pub struct AttrType {
    pub quantifier: Quantifier,
    pub primitives: &'static[PrimitiveType],
}

///
/// Type for an attribute value
///
pub enum PrimitiveType {
    // Empty value, usually means "true"
    Empty,
    True,
    False,
    Enum(EnumType),
    // Any string:
    String,
    // Any number:
    Number,

    // FAKE VALUES, REMOVE!!!
    Bool,

    BoolOrEmptyString,

    // The property is a boolean. The default value of this property is something other than false, so false must persist. The value can hold a string (as is the case with ariaChecked and its 'mixed' value)
    OverloadedBool,

    // The property is a boolean. The default value of this property is something other than false, so false must persist. The value can hold a string (as is the case with ariaChecked and its 'mixed' value)
    BooleanIsh,

    // The property is number. These values can sometimes hold a string
}

pub enum EnumType {
    Mixed,
}

pub enum Quantifier {
    One,
    CommaSeparated,
    SpaceSeparated,
    CommaOrSpaceSeparated
}
