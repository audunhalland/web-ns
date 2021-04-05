//! Web namespaces.
//!
//! # Usage
//!
//! ```
//! use doml::*;
//! use doml_web::html5::HTML5_NS;
//!
//! let element = HTML5_NS.element_by_local_name("div").unwrap();
//! ```
//!

#![forbid(unsafe_code)]

pub mod html5;

mod attr;
mod new;
mod static_unicase;
mod static_web_attr;

use new::{Attribute, Element};

pub mod schema {
    pub mod html5;
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum WebNS {
    HTML5,
}

impl WebNS {
    fn name(&self) -> &'static str {
        match self {
            Self::HTML5 => "html5",
        }
    }
}

///
/// A specific document Schema used on the web.
///
pub enum Schema {
    Html5,
}

struct Private;

pub trait WebNamespace {
    fn element_by_local_name(&self, _local_name: &str) -> Result<Element, Error>;

    fn attribute_by_local_name(&self, _: &Element, local_name: &str) -> Result<Attribute, Error>;

    fn attribute_by_property(&self, property_name: &str) -> Result<Attribute, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    InvalidAttributeValue,
}

///
/// Access the JS DOM property name of an attribute.
///
/// # Example
/// ```
/// assert_eq!(attribute_property_name(&crate::html5::attributes::CLASS), Some("className"));
/// ```
///
pub fn attribute_property_name(attribute: &Attribute) -> Option<&str> {
    todo!()
    /*
    let (class, id) = attribute.name().instance();

    match id {
        Some(static_id) => {
            if let Some(static_web_attr_class) =
                class.downcast_ref::<static_web_attr::StaticWebAttrNS>()
            {
                Some(static_web_attr_class.property_name(static_id))
            } else {
                None
            }
        }
        None => None,
    }
    */
}
