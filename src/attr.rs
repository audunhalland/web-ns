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

pub struct AttributeInfo<'a> {
    pub(crate) web_ns: WebNS,
    pub(crate) attr_type: attr_type::AttrType,
    pub(crate) property: Option<&'a str>,
}

impl<'a> AttributeInfo<'a> {
    pub fn web_namespace(&self) -> &dyn crate::WebNamespace {
        self.web_ns.web_namespace()
    }

    pub fn attr_type(&self) -> attr_type::AttrType {
        self.attr_type
    }

    pub fn property(&self) -> Option<&'a str> {
        self.property
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
