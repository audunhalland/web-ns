//! Typed web attributes.
//!
//! The `Attribute` type implements `Hash` and `Ord`, so this can be
//! conveniently used as map keys.
//!
//! Known, internal attributes allocate no memory upon being parsed.
//!
pub mod attr_type;

pub(crate) mod dataset;

mod value;

use crate::Error;

///
/// Basic trait for attributes.
///
pub trait Attribute {
    fn attr_type(&self) -> attr_type::AttrType;
}

///
/// Trait for deserialization of attribute values.
///
pub trait DeserializeAttributeValue {
    fn deserialize_attribute_value<S>(&self, input: Option<S>) -> Result<AttributeValue, Error>
    where
        S: Into<String> + AsRef<str>;
}

impl<A> DeserializeAttributeValue for A
where
    A: Attribute,
{
    fn deserialize_attribute_value<S>(&self, input: Option<S>) -> Result<AttributeValue, Error>
    where
        S: Into<String> + AsRef<str>,
    {
        value::parse_attribute(input, self.attr_type())
    }
}

///
/// Trait for serialization of attribute values.
///
pub trait SerializeAttributeValue {
    fn serialize_attribute_value(&self, value: &AttributeValue) -> SerializedAttributeValue;
}

impl<A> SerializeAttributeValue for A
where
    A: Attribute,
{
    fn serialize_attribute_value(&self, value: &AttributeValue) -> SerializedAttributeValue {
        value::serialize_attribute_value(value, self.attr_type())
    }
}

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
