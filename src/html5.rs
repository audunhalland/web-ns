//! HTML5 implementation

use dyn_symbol::Symbol;

use crate::attr::attr_type::flags;
use crate::attr::attr_type::AttrType;
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
    fn parse_data_attribute(&self, name: &str) -> Result<Symbol, Error> {
        if name.len() > 5 && unicase::UniCase::new(&name[..5]) == unicase::UniCase::new("data-") {
            let strbuf = format!(
                "data-{}data{}{}",
                &name[5..],
                name.chars().nth(5).unwrap().to_uppercase(),
                &name[6..]
            );
            Ok(Symbol::Dynamic(Box::new(data_attr::DataAttr {
                strbuf,
                buf_property_start: name.len(),
                attr_type: AttrType(flags::STRING),
            })))
        } else {
            Err(Error::InvalidAttribute)
        }
    }
}

impl super::WebNamespace for Html5Namespace {
    fn element_by_local_name(&self, name: &str) -> Result<Symbol, Error> {
        tags::__TAG_LOOKUP_TABLE.tag_by_local_name(name)
    }

    fn attribute_by_local_name(&self, _: &Symbol, name: &str) -> Result<Symbol, Error> {
        attributes::__ATTR_LOOKUP_TABLES
            .attribute_by_local_name(name)
            .or_else(|_| self.parse_data_attribute(name))
    }

    fn attribute_by_property(&self, property_name: &str) -> Result<Symbol, Error> {
        attributes::__ATTR_LOOKUP_TABLES.attribute_by_property_name(property_name)
    }
}

pub(crate) mod data_attr {
    use super::*;
    use dyn_symbol::namespace::Dynamic;

    #[derive(Clone)]
    pub struct DataAttr {
        pub strbuf: String,
        pub buf_property_start: usize,
        pub attr_type: AttrType,
    }

    impl DataAttr {
        pub fn attr_name(&self) -> &str {
            &self.strbuf[..self.buf_property_start]
        }

        pub fn property_name(&self) -> &str {
            &self.strbuf[self.buf_property_start..]
        }
    }

    impl Dynamic for DataAttr {
        fn namespace_name(&self) -> &str {
            "html5::data"
        }

        fn symbol_name(&self) -> &str {
            &self.strbuf[..self.buf_property_start]
        }

        fn dyn_clone(&self) -> Box<dyn Dynamic> {
            Box::new(self.clone())
        }

        fn dyn_eq(&self, rhs: &dyn Dynamic) -> bool {
            self.attr_name() == rhs.downcast_ref::<Self>().unwrap().attr_name()
        }

        fn dyn_cmp(&self, rhs: &dyn Dynamic) -> std::cmp::Ordering {
            let rhs_attr_name = rhs.downcast_ref::<Self>().unwrap().attr_name();
            self.attr_name().cmp(rhs_attr_name)
        }

        fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
            state.write(self.attr_name().as_bytes());
            state.write_u8(0xff)
        }
    }
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
        let element = html5::tags::DIV;

        if let Ok(attribute) = html5::HTML5_NS.attribute_by_local_name(&element, name) {
            let expected = expected.expect("Expected no match, but there was a match");
            assert_eq!(attribute.name(), expected.0);
            assert_eq!(attribute_property_name(&attribute).unwrap(), expected.1);
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
        // let mut hashmap = std::collections::HashMap::new();
        // hashmap.insert(attributes::VLINK.clone(), 42);
    }
}
