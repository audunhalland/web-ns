//! Typed web attributes.
//!
//! The `Attribute` type implements `Hash` and `Ord`, so this can be
//! conveniently used as map keys.
//!
//! Known, internal attributes allocate no memory upon being parsed.
//!
pub mod attr_impl;
pub mod attr_type;

pub(crate) mod data;

mod value;

use dyn_symbol::Symbol;
use std::convert::TryFrom;

use crate::static_web_attr::StaticWebAttrSymbolNamespace;
use crate::{Error, WebNS};

///
/// Metadata about an attribute.
///
/// Due to the current design of attributes, AttributeInfo can only be acquired
/// through calling [TryFrom] on a [dyn_symbol::Symbol] reference.
///
/// # Usage
///
/// ```
/// use web_ns::*;
/// use std::convert::TryFrom;
///
/// let info = AttributeInfo::try_from(&html5::attributes::CLASS).unwrap();
/// assert_eq!(info.property(), Some("className"));
/// ```
///
pub struct AttributeInfo<'a> {
    pub(crate) web_ns: WebNS,
    pub(crate) attr_type: attr_type::AttrType,
    pub(crate) property: Option<&'a str>,
}

impl<'a> AttributeInfo<'a> {
    /// Get the originating namespace of the attribute.
    pub fn web_namespace(&self) -> &dyn crate::WebNamespace {
        self.web_ns.web_namespace()
    }

    /// Get the attribute type.
    pub fn attr_type(&self) -> attr_type::AttrType {
        self.attr_type
    }

    /// Access the JS property name of the attribute.
    pub fn property(&self) -> Option<&'a str> {
        self.property
    }

    /// Parse a _value_ of the attribute, according to the type of the attribute.
    ///
    /// # Usage
    ///
    /// ```
    /// use web_ns::*;
    /// use std::convert::TryFrom;
    ///
    /// let info = AttributeInfo::try_from(&html5::attributes::CLASS).unwrap();
    /// let value = info.deserialize_value(Some("foo bar")).unwrap();
    ///
    /// assert_eq!(value, AttributeValue::Multi(vec!["foo".to_string(), "bar".to_string()]));
    /// ```
    pub fn deserialize_value<S>(&self, value: Option<S>) -> Result<AttributeValue, Error>
    where
        S: Into<String> + AsRef<str>,
    {
        value::parse_attribute(value, self.attr_type)
    }

    /// Parse aa _value_ of the attribute, according to the type of the attribute.
    ///
    /// # Usage
    ///
    /// ```
    /// use web_ns::*;
    /// use std::convert::TryFrom;
    ///
    /// let info = AttributeInfo::try_from(&html5::attributes::CLASS).unwrap();
    /// let value = info.serialize_value(&AttributeValue::Multi(vec!["foo".to_string(), "bar".to_string()]));
    ///
    /// assert_eq!(value, SerializedAttributeValue::String("foo bar".to_string()));
    /// ```
    pub fn serialize_value(&self, value: &AttributeValue) -> SerializedAttributeValue {
        value::serialize_attribute_value(&value, self.attr_type)
    }
}

impl<'a> TryFrom<&'a Symbol> for AttributeInfo<'a> {
    type Error = crate::Error;

    fn try_from(value: &'a Symbol) -> Result<Self, Self::Error> {
        if let Some((static_attr_ns, id)) = value.downcast_static::<StaticWebAttrSymbolNamespace>()
        {
            return Ok(static_attr_ns.attribte_info(id));
        }

        if let Some(data_attr) = value.downcast_dyn::<data::DataAttr>() {
            return Ok(data_attr.attribute_info());
        }

        Err(Error::InvalidAttribute)
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
