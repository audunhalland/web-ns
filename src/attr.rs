pub mod attr_impl;
pub mod attr_type;
mod value;

use crate::schema;
use crate::Schema;

use attr_impl::*;
use attr_type::*;

///
/// A web attribute.
///
#[derive(Clone)]
pub struct Attribute(attr_impl::AttrImpl);

impl Attribute {
    ///
    /// Access the markup attribute name of this attribute.
    ///
    pub fn attribute(&self) -> &str {
        match &self.0 {
            AttrImpl::Internal(attr) => attr.attribute,
            AttrImpl::Data(attr) => &attr.strbuf[..attr.buf_property_start],
        }
    }

    ///
    /// Access the JS property name of this attribute.
    ///
    pub fn property(&self) -> &str {
        match &self.0 {
            AttrImpl::Internal(attr) => attr.property,
            AttrImpl::Data(attr) => &attr.strbuf[attr.buf_property_start..],
        }
    }

    // FIXME: proper error type
    pub fn encode_value<S>(&self, value: Option<S>) -> Result<AttributeValue, crate::Error>
    where
        S: Into<String> + AsRef<str>,
    {
        value::encode(value, self.attr_type())
    }

    fn attr_type(&self) -> AttrType {
        match &self.0 {
            AttrImpl::Internal(attr) => attr.attr_type,
            AttrImpl::Data(attr) => attr.attr_type,
        }
    }
}

impl PartialEq<Attribute> for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.attribute() == other.attribute()
    }
}

impl Eq for Attribute {}

impl std::hash::Hash for Attribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.attribute().hash(state);
    }
}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.attribute().partial_cmp(other.attribute())
    }
}

impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attribute().cmp(other.attribute())
    }
}

///
/// A typed attribute value.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum AttributeValue {
    True,
    False,
    String(String),
    Multi(Vec<String>),
}

///
/// Tries to parse an attribute name. Official web attributes
/// should not allocate any memory.
///
pub fn parse_attribute(attribute: &str, schema: Schema) -> Option<Attribute> {
    match schema {
        Schema::Html5 => match schema::html::attrs::internal_attr_by_name(attribute) {
            Some(internal_attr) => Some(Attribute(AttrImpl::Internal(internal_attr))),
            None => {
                if attribute.len() > 5
                    && unicase::UniCase::new(&attribute[..5]) == unicase::UniCase::new("data-")
                {
                    let strbuf = format!(
                        "data-{}data{}{}",
                        &attribute[5..],
                        attribute.chars().nth(5).unwrap().to_uppercase(),
                        &attribute[6..]
                    );
                    Some(Attribute(AttrImpl::Data(Box::new(DataAttr {
                        strbuf,
                        buf_property_start: attribute.len(),
                        attr_type: AttrType(flags::STRING),
                    }))))
                } else {
                    None
                }
            }
        },
    }
}

pub fn parse_property(property: &str, schema: Schema) -> Option<Attribute> {
    match schema {
        Schema::Html5 => match schema::html::attrs::internal_attr_by_property(property) {
            Some(internal_attr) => Some(Attribute(AttrImpl::Internal(internal_attr))),
            None => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_html_attribute(name: &str, expected: Option<(&str, &str)>) {
        if let Some(attribute) = parse_attribute(name, Schema::Html5) {
            let expected = expected.expect("Expected no match, but there was a match");
            assert_eq!(attribute.attribute(), expected.0);
            assert_eq!(attribute.property(), expected.1);
        } else {
            assert!(expected.is_none());
        }
    }

    #[test]
    fn parse_html5_attributes() {
        test_html_attribute("accept-charset", Some(("accept-charset", "acceptCharset")));
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
}
