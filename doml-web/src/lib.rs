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
mod static_unicase;
mod static_web_attr;

pub mod schema {
    pub mod html5;
}

#[derive(Eq, PartialEq)]
enum WebNS {
    HTML5,
}

///
/// A specific document Schema used on the web.
///
pub enum Schema {
    Html5,
}

struct Private;

pub trait WebNamespace: doml::Namespace {
    fn attribute_by_property(
        &self,
        property_name: &str,
    ) -> Result<doml::attribute::Attribute, doml::Error>;
}

///
/// Access the JS DOM property name of an attribute.
///
/// # Example
/// ```
/// assert_eq!(attribute_property_name(&crate::html5::attributes::CLASS), Some("className"));
/// ```
///
pub fn attribute_property_name(attribute: &doml::attribute::Attribute) -> Option<&str> {
    let (class, id) = attribute.instance();

    match id {
        Some(static_id) => {
            if let Some(static_web_attr_class) =
                class.downcast_ref::<static_web_attr::StaticWebAttrClass>()
            {
                Some(static_web_attr_class.property_name(static_id))
            } else {
                None
            }
        }
        None => None,
    }
}
