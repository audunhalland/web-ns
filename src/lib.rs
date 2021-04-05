//! Web namespaces.
//!
//! # Usage
//!
//! ```
//! use web_ns::*;
//!
//! let element = html5::HTML5_NS.element_by_local_name("div").unwrap();
//! ```
//!

#![forbid(unsafe_code)]

use dyn_symbol::Symbol;

pub mod html5;

mod attr;
mod static_unicase;
mod static_web_attr;
mod symbols;

#[derive(Clone, Copy, Eq, PartialEq)]
enum WebNS {
    HTML5 = 0,
}

impl WebNS {
    fn name(&self) -> &'static str {
        match self {
            Self::HTML5 => "html5",
        }
    }
}

struct Private;

pub trait WebNamespace {
    fn element_by_local_name(&self, _local_name: &str) -> Result<Symbol, Error>;

    fn attribute_by_local_name(&self, _: &Symbol, local_name: &str) -> Result<Symbol, Error>;

    fn attribute_by_property(&self, property_name: &str) -> Result<Symbol, Error>;
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
/// use web_ns::*;
///
/// assert_eq!(attribute_property_name(&html5::attributes::CLASS), Some("className"));
/// ```
///
pub fn attribute_property_name(attribute: &Symbol) -> Option<&str> {
    use html5::data_attr::DataAttr;
    use static_web_attr::StaticWebAttrSymbolNamespace;

    if let Some((html5_ns, id)) = attribute.downcast_static::<StaticWebAttrSymbolNamespace>() {
        return Some(html5_ns.property_name(id));
    }

    if let Some(data_attr) = attribute.downcast_dyn::<DataAttr>() {
        return Some(data_attr.property_name());
    }

    return None;
}
