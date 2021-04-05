//! Typed web attributes.
//!
//! The `Attribute` type implements `Hash` and `Ord`, so this can be
//! conveniently used as map keys.
//!
//! Known, internal attributes allocate no memory upon being parsed.
//!
pub mod attr_impl;
pub mod attr_type;

mod value;

use crate::Error;
use crate::Schema;

use attr_impl::*;
use attr_type::*;

///
/// A typed attribute value.
///
/// This is the output of the Attribute::parse method.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum AttributeValue {
    True,
    False,
    String(String),
    Multi(Vec<String>),
}

///
/// A serialized attribute value, for building DOM documents.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SerializedAttributeValue {
    /// The entire attribute can be be omitted from markup.
    Omitted,
    /// The attribute should be empty/valueless.
    /// `<foo attribute />`
    Empty,
    /// The attribute has a string value.
    /// `<foo attribute="bar" />`
    String(String),
}

/*
///
/// Parse a DOM property name.
///
/// # Usage
///
/// ```
/// use web_ns::Schema;
/// use web_ns::attr::parse_property;
///
/// let attr = parse_property("className", Schema::Html5).unwrap();
/// assert_eq!(attr.attribute(), "class");
/// ```
///
pub fn parse_property(property: &str, schema: Schema) -> Result<Attribute, Error> {
    match schema {
        Schema::Html5 => match schema::html5::attr::internal_attr_by_property(property) {
            Some(internal_attr) => Ok(Attribute(AttrImpl::Internal(internal_attr))),
            None => Err(Error::InvalidAttribute),
        },
    }
}
*/
