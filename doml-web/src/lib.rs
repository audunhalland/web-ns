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

use doml::attribute::Attribute as DomlAttribute;
use doml::element::Element as DomlElement;

mod attr;

mod static_unicase;

pub mod schema {
    pub mod html5;
}

///
/// A specific document Schema used on the web.
///
pub enum Schema {
    Html5,
}

struct Private;

pub mod html5 {
    //! HTML5 implementation

    use super::*;

    pub mod attrs {
        //! Attribute definitions for HTML5
        struct StaticAttrClass;

        impl doml::attribute::AttributeClass for StaticAttrClass {
            fn namespace(&self) -> &'static dyn doml::Namespace {
                &super::HTML5_NS
            }

            fn local_name(&self, static_id: Option<usize>) -> &str {
                let static_id = static_id.unwrap();
                WEB_ATTRS[static_id].name
            }
        }

        const STATIC_ATTR_CLASS: StaticAttrClass = StaticAttrClass;

        include!(concat!(env!("OUT_DIR"), "/codegen_static_html_attrs.rs"));
    }

    /// A [doml::Namespace] implementation for HTML5.
    pub struct Html5Namespace(Private);

    /// The global [Html5Namespace] instance.
    pub const HTML5_NS: Html5Namespace = Html5Namespace(Private);

    impl doml::Namespace for Html5Namespace {
        fn element_by_local_name(&self, _local_name: &str) -> Result<DomlElement, doml::Error> {
            todo!()
        }

        fn attribute_by_local_name(
            &self,
            _element: &DomlElement,
            local_name: &str,
        ) -> Result<DomlAttribute, doml::Error> {
            attrs::ATTRIBUTE_UNICASE_PHF
                .get(&unicase::UniCase::ascii(local_name))
                .map(|web_attr| DomlAttribute::new_static(&web_attr.static_attribute))
                .ok_or_else(|| doml::Error::InvalidAttribute)
        }
    }
}
