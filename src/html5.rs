//! HTML5 implementation

use crate::attr::attr_type::flags;
use crate::attr::attr_type::AttrType;
use crate::Error;

use super::*;

mod tags {
    //! Tag definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_tag_html_enums.rs"));
}

mod attributes {
    //! Attribute definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_attr_html_enums.rs"));
}

pub use attributes::HtmlAttr;
pub use tags::HtmlTag;

/// A [web_ns::WebNamespace] implementation for HTML5.
pub struct Html5Namespace(Private);

/// The global [Html5Namespace] instance.
pub const HTML5_NS: Html5Namespace = Html5Namespace(Private);

impl super::web::WebNamespace for Html5Namespace {
    fn name(&self) -> &'static str {
        "html5"
    }
}

fn parse_data_attribute(name: &str) -> Result<attr::dataset::DataAttr, Error> {
    if name.len() > 5 && unicase::UniCase::new(&name[..5]) == unicase::UniCase::new("data-") {
        let strbuf = format!(
            "data-{}data{}{}",
            &name[5..],
            name.chars().nth(5).unwrap().to_uppercase(),
            &name[6..]
        );
        Ok(attr::dataset::DataAttr {
            strbuf,
            buf_property_start: name.len(),
            attr_type: AttrType(flags::STRING),
        })
    } else {
        Err(Error::InvalidAttribute)
    }
}

fn parse_data_property(name: &str) -> Result<attr::dataset::DataAttr, Error> {
    if name.len() > 4 && name.starts_with("data") {
        let strbuf = format!(
            "data-{}{}{}",
            name.chars().nth(4).unwrap().to_lowercase(),
            &name[5..],
            name
        );
        Ok(attr::dataset::DataAttr {
            strbuf,
            buf_property_start: name.len() + 1,
            attr_type: AttrType(flags::STRING),
        })
    } else {
        Err(Error::InvalidAttribute)
    }
}

impl crate::TagByLocalName<tags::HtmlTag> for Html5Namespace {
    fn tag_by_local_name(&self, local_name: &str) -> Result<tags::HtmlTag, Error> {
        tags::STATIC_LOCAL_NAME_LOOKUP
            .get(&unicase::UniCase::ascii(local_name))
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

impl crate::TagByLocalName<crate::web::Tag> for Html5Namespace {
    fn tag_by_local_name(&self, local_name: &str) -> Result<crate::web::Tag, Error> {
        self.tag_by_local_name(local_name)
            .map(|tag| super::web::Tag::Html5(tag))
    }
}

impl crate::AttrByLocalName<attributes::HtmlAttr> for tags::HtmlTag {
    fn attr_by_local_name(&self, local_name: &str) -> Result<attributes::HtmlAttr, Error> {
        attributes::STATIC_LOCAL_NAME_LOOKUP
            .get(&unicase::UniCase::ascii(local_name))
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
            .or_else(|_| {
                parse_data_attribute(local_name)
                    .map(|attr| attributes::HtmlAttr::Dataset(Box::new(attr)))
            })
    }
}

impl crate::AttrByProperty<attributes::HtmlAttr> for tags::HtmlTag {
    fn attr_by_property(&self, property: &str) -> Result<attributes::HtmlAttr, Error> {
        attributes::STATIC_PROPERTY_LOOKUP
            .get(property)
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
            .or_else(|_| {
                parse_data_property(property)
                    .map(|data| attributes::HtmlAttr::Dataset(Box::new(data)))
            })
    }
}

impl crate::AttrByLocalName<crate::web::Attr> for tags::HtmlTag {
    fn attr_by_local_name(&self, local_name: &str) -> Result<crate::web::Attr, Error> {
        self.attr_by_local_name(local_name)
            .map(|attr| super::web::Attr::Html5(attr))
    }
}

impl crate::AttrByProperty<crate::web::Attr> for tags::HtmlTag {
    fn attr_by_property(&self, property: &str) -> Result<crate::web::Attr, Error> {
        self.attr_by_property(property)
            .map(|attr| super::web::Attr::Html5(attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html5_static_attribute_to_property_name() {
        let attribute = html5::attributes::HtmlAttr::Class;

        assert_eq!(attribute.property_name(), "className");
    }

    fn test_html_attribute(name: &str, expected: Option<(&str, &str)>) {
        let element = tags::HtmlTag::Div;
        let result: Result<attributes::HtmlAttr, _> = element.attr_by_local_name(name);

        if let Ok(attribute) = result {
            let expected = expected.expect("Expected no match, but there was a match");
            assert_eq!(attribute.local_name(), expected.0);

            assert_eq!(attribute.property_name(), expected.1);

            let attr_by_prop: attributes::HtmlAttr = element.attr_by_property(expected.1).unwrap();

            assert_eq!(attr_by_prop.local_name(), expected.0);
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
        let tag = tags::HtmlTag::Div;

        let attr: attributes::HtmlAttr = tag.attr_by_property("className").unwrap();
        assert_eq!(attr, attributes::HtmlAttr::Class);

        let attr: attributes::HtmlAttr = tag.attr_by_property("src").unwrap();
        assert_ne!(attr, attributes::HtmlAttr::Class);

        let result: Result<attributes::HtmlAttr, _> = tag.attr_by_property("ClassName");
        assert!(result.is_err());

        let result: Result<attributes::HtmlAttr, _> = tag.attr_by_property("foobar");
        assert!(result.is_err());
    }

    #[test]
    fn properties_in_hashmap() {
        let mut hashmap = std::collections::HashMap::new();
        hashmap.insert(attributes::HtmlAttr::Vlink, 42);

        assert_eq!(hashmap.get(&attributes::HtmlAttr::Vlink), Some(&42));
    }
}
