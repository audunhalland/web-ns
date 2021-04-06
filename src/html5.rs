//! HTML5 implementation

use dyn_symbol::Symbol;

use crate::attr::attr_type::flags;
use crate::attr::attr_type::AttrType;
use crate::symbols::attr::__ATTR_SYMBOL_NS;
use crate::Error;

use super::*;

pub mod tags {
    //! Tag definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_static_html_tags.rs"));
}

pub mod attributes {
    //! Attribute definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_static_html_attrs.rs"));
}

/// A [doml::Namespace] implementation for HTML5.
pub struct Html5Namespace(Private);

/// The global [Html5Namespace] instance.
pub const HTML5_NS: Html5Namespace = Html5Namespace(Private);

impl Html5Namespace {
    /// Look up an attribute by its DOM JavaScript property name.
    pub fn attribute_by_property(&self, property_name: &str) -> Result<Symbol, Error> {
        attributes::__ATTR_LOOKUP_TABLES
            .attribute_by_property_name(property_name)
            .map(|id| Symbol::Static(&__ATTR_SYMBOL_NS, id))
    }

    fn parse_data_attribute(&self, name: &str) -> Result<Symbol, Error> {
        if name.len() > 5 && unicase::UniCase::new(&name[..5]) == unicase::UniCase::new("data-") {
            let strbuf = format!(
                "data-{}data{}{}",
                &name[5..],
                name.chars().nth(5).unwrap().to_uppercase(),
                &name[6..]
            );
            let attr = attr::data::DataAttr {
                strbuf,
                buf_property_start: name.len(),
                attr_type: AttrType(flags::STRING),
            };

            Ok(Symbol::Dynamic(Box::new(attr)))
        } else {
            Err(Error::InvalidAttribute)
        }
    }
}

impl super::WebNamespace for Html5Namespace {
    fn name(&self) -> &'static str {
        "html5"
    }

    fn element_by_local_name(&self, name: &str) -> Result<Symbol, Error> {
        tags::__TAG_LOOKUP_TABLE.tag_by_local_name(name)
    }

    fn attribute_by_local_name(&self, _: &Symbol, name: &str) -> Result<Symbol, Error> {
        attributes::__ATTR_LOOKUP_TABLES
            .attribute_by_local_name(name)
            .map(|id| Symbol::Static(&__ATTR_SYMBOL_NS, id))
            .or_else(|_| self.parse_data_attribute(name))
    }
}

#[cfg(test)]
mod tests {
    use super::WebNamespace;
    use super::*;

    use std::convert::TryFrom;

    #[test]
    fn html5_static_attribute_to_property_name() {
        let attribute = html5::attributes::CLASS;

        let info = attr::AttributeInfo::try_from(&attribute).unwrap();
        assert_eq!(info.property(), Some("className"));
    }

    fn test_html_attribute(name: &str, expected: Option<(&str, &str)>) {
        let element = html5::tags::DIV;

        if let Ok(attribute) = html5::HTML5_NS.attribute_by_local_name(&element, name) {
            let expected = expected.expect("Expected no match, but there was a match");
            assert_eq!(attribute.name(), expected.0);

            let info = attr::AttributeInfo::try_from(&attribute).unwrap();
            assert_eq!(info.property, Some(expected.1));
        } else {
            assert!(expected.is_none());
        }
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
        let mut hashmap = std::collections::HashMap::new();
        hashmap.insert(attributes::VLINK.clone(), 42);

        assert_eq!(hashmap.get(&attributes::VLINK), Some(&42));
    }
}
