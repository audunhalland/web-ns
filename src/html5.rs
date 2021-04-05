//! HTML5 implementation

use std::hash::Hasher;

use crate::new::{Attribute, Element};
use crate::Error;

use super::*;

use crate::attr::attr_type::AttrType;

pub mod attributes {
    //! Attribute definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_static_html_attrs.rs"));
}

/// A [doml::Namespace] implementation for HTML5.
pub struct Html5Namespace(Private);

/// The global [Html5Namespace] instance.
pub const HTML5_NS: Html5Namespace = Html5Namespace(Private);

impl super::WebNamespace for Html5Namespace {
    fn element_by_local_name(&self, _local_name: &str) -> Result<Element, Error> {
        todo!()
    }

    fn attribute_by_local_name(&self, _: &Element, local_name: &str) -> Result<Attribute, Error> {
        attributes::__ATTR_NS.attribute_by_local_name(local_name)
    }

    fn attribute_by_property(&self, property_name: &str) -> Result<Attribute, Error> {
        attributes::__ATTR_NS.attribute_by_property_name(property_name)
    }
}

pub struct DataAttr {
    pub strbuf: String,
    pub buf_property_start: usize,
    pub attr_type: AttrType,
}

#[cfg(test)]
mod tests {
    use super::WebNamespace;
    use super::*;

    #[test]
    fn html5_static_attribute_to_property_name() {
        let attribute = html5::attributes::CLASS;

        assert_eq!(attribute_property_name(&attribute), Some("className"));
    }

    fn test_html_attribute(name: &str, expected: Option<(&str, &str)>) {
        todo!();
        /*
        let element = doml::any_ns::ANY_NS.element_by_local_name("test").unwrap();

        if let Ok(attribute) = html5::HTML5_NS.attribute_by_local_name(&element, name) {
            let expected = expected.expect("Expected no match, but there was a match");
            assert_eq!(attribute.local_name(), expected.0);
            assert_eq!(attribute_property_name(&attribute).unwrap(), expected.1);
        } else {
            assert!(expected.is_none());
        }
        */
    }

    #[test]
    fn parse_html5_attributes() {
        test_html_attribute("accept-charset", Some(("accept-charset", "acceptCharset")));
        test_html_attribute("ACCEPT-CHARSET", Some(("accept-charset", "acceptCharset")));
        test_html_attribute("class", Some(("class", "className")));
        test_html_attribute("foobar", None);
        test_html_attribute("data", Some(("data", "data")));
        test_html_attribute("data-", None);
        test_html_attribute("data-", None);
        test_html_attribute("data-1", Some(("data-1", "data1")));
        test_html_attribute("Data-1", Some(("data-1", "data1")));
        test_html_attribute("data-f", Some(("data-f", "dataF")));
        test_html_attribute("data-foobar", Some(("data-foobar", "dataFoobar")));
        test_html_attribute("data-fooBar-1", Some(("data-fooBar-1", "dataFooBar-1")));
    }

    #[test]
    fn lookup_html_property() {
        assert_eq!(
            HTML5_NS.attribute_by_property("className").unwrap(),
            attributes::CLASS
        );
        assert_ne!(
            HTML5_NS.attribute_by_property("src").unwrap(),
            attributes::CLASS
        );
        assert!(HTML5_NS.attribute_by_property("ClassName").is_err());
        assert!(HTML5_NS.attribute_by_property("foobar").is_err());
    }

    #[test]
    fn properties_in_hashmap() {
        // let mut hashmap = std::collections::HashMap::new();
        // hashmap.insert(attributes::VLINK.clone(), 42);
    }
}
