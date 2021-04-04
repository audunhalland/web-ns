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

mod metadata {
    pub const PROPERTY: &str = "web_property";
}

pub fn attribute_property_name(attribute: &doml::attribute::Attribute) -> Option<&str> {
    attribute.get_metadata(metadata::PROPERTY)
}

pub mod html5 {
    //! HTML5 implementation

    use std::any::Any;

    use super::*;

    pub mod attrs {
        //! Attribute definitions for HTML5
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

    struct DataAttr;

    impl doml::attribute::AttributeClass for DataAttr {
        fn namespace(&self) -> &'static dyn doml::Namespace {
            &HTML5_NS
        }

        fn local_name(&self, _: Option<usize>) -> &str {
            "data"
        }

        fn metadata<'a>(&'a self, _: Option<usize>, _: &str) -> Option<&'a str> {
            None
        }
    }

    /*
    fn class_to_any(class: &dyn doml::attribute::AttributeClass) -> &dyn std::any::Any {
        // &class
        panic!()
    }

    fn test_downcast(attr: &doml::attribute::Attribute) {
        use std::any::Any;
        let (instance, id) = attr.instance();

        // if let Some(data_attr) = instance.downcast_ref::<DataAttr>() {}

        instance.type_id();
    }
    */
}
