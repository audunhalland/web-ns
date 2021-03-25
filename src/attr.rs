//! Typed web attributes.
//!
//! The `Attribute` type implements `Hash` and `Ord`, so this can be
//! conveniently used as map keys.
//!
//! Known, internal attributes allocate no memory upon being parsed.
//!
pub mod attr_impl;
pub mod attr_type;
mod value;

use crate::schema;
use crate::Schema;

use attr_impl::*;
use attr_type::*;

///
/// A typed attribute used in web documents.
///
#[derive(Clone)]
pub struct Attribute(attr_impl::AttrImpl);

impl Attribute {
    ///
    /// Access the canonical markup attribute name of this attribute.
    ///
    /// # Usage
    ///
    /// ```
    /// use web_schema::Schema;
    /// use web_schema::attr::parse_attribute;
    ///
    /// let attr = parse_attribute("ClAsS", Schema::Html5).unwrap();
    /// assert_eq!(attr.attribute(), "class");
    /// ```
    ///
    pub fn attribute(&self) -> &str {
        match &self.0 {
            AttrImpl::Internal(attr) => attr.attribute.as_ref(),
            AttrImpl::Data(attr) => &attr.strbuf[..attr.buf_property_start],
        }
    }

    ///
    /// Access the JS property name of this attribute.
    ///
    /// # Usage
    ///
    /// ```
    /// use web_schema::Schema;
    /// use web_schema::attr::parse_attribute;
    ///
    /// let attr = parse_attribute("ClAsS", Schema::Html5).unwrap();
    /// assert_eq!(attr.property(), "className");
    /// ```
    ///
    pub fn property(&self) -> &str {
        match &self.0 {
            AttrImpl::Internal(attr) => attr.property,
            AttrImpl::Data(attr) => &attr.strbuf[attr.buf_property_start..],
        }
    }

    ///
    /// Parse a string-based _value_ of this attribute.
    /// The Option is there to represent an attribute with no value.
    ///
    /// # Usage
    ///
    /// ```
    /// use web_schema::Schema;
    /// use web_schema::attr::*;
    ///
    /// let attr = parse_attribute("class", Schema::Html5).unwrap();
    /// let value = attr.parse_attribute_value(Some("foo bar")).unwrap();
    /// assert_eq!(value, AttributeValue::Multi(vec!["foo".to_string(), "bar".to_string()]));
    /// ```
    ///
    pub fn parse_attribute_value<S>(&self, value: Option<S>) -> Result<AttributeValue, crate::Error>
    where
        S: Into<String> + AsRef<str>,
    {
        value::parse_attribute(value, self.attr_type())
    }

    ///
    /// Serialize an attribute value, following the rules on how
    /// that value should be serialized given the schema that the attribute originates from.
    ///
    pub fn serialize_attribute_value(&self, value: &AttributeValue) -> SerializedAttributeValue {
        value::serialize_attribute_value(value, self.attr_type())
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
/// This is the output of the Attribute::parse method.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum AttributeValue {
    True,
    False,
    String(String),
    Multi(Vec<String>),
}

///
/// A serialized attribute value, for building DOM documents.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SerializedAttributeValue {
    /// The entire attribute can be be omitted from markup.
    Omitted,
    /// The attribute should be empty/valueless.
    /// `<foo attribute />`
    Empty,
    /// The attribute has a string value.
    /// `<foo attribute="bar" />`
    String(String),
}

///
/// Parse an attribute name. Known web attributes do not allocate any memory.
/// For known attributes, the matching is case-insensitive.
///
/// ```
/// use web_schema::Schema;
/// use web_schema::attr::parse_attribute;
///
/// let attr = parse_attribute("data-foobar", Schema::Html5).unwrap();
/// assert_eq!(attr.attribute(), "data-foobar");
/// assert_eq!(attr.property(), "dataFoobar");
/// ```
///
pub fn parse_attribute(attribute: &str, schema: Schema) -> Result<Attribute, crate::Error> {
    match schema {
        Schema::Html5 => match schema::html::attrs::internal_attr_by_name(attribute) {
            Some(internal_attr) => Ok(Attribute(AttrImpl::Internal(internal_attr))),
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
                    Ok(Attribute(AttrImpl::Data(Box::new(DataAttr {
                        strbuf,
                        buf_property_start: attribute.len(),
                        attr_type: AttrType(flags::STRING),
                    }))))
                } else {
                    Err(crate::Error::InvalidName)
                }
            }
        },
    }
}

///
/// Parse a DOM property name.
///
/// # Usage
///
/// ```
/// use web_schema::Schema;
/// use web_schema::attr::parse_property;
///
/// let attr = parse_property("className", Schema::Html5).unwrap();
/// assert_eq!(attr.attribute(), "class");
/// ```
///
pub fn parse_property(property: &str, schema: Schema) -> Result<Attribute, crate::Error> {
    match schema {
        Schema::Html5 => match schema::html::attrs::internal_attr_by_property(property) {
            Some(internal_attr) => Ok(Attribute(AttrImpl::Internal(internal_attr))),
            None => Err(crate::Error::InvalidName),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_html_attribute(name: &str, expected: Option<(&str, &str)>) {
        if let Ok(attribute) = parse_attribute(name, Schema::Html5) {
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
        assert!(parse_property("className", Schema::Html5).is_ok());
        assert!(parse_property("ClassName", Schema::Html5).is_err());
        assert!(parse_property("foobar", Schema::Html5).is_err());
    }
}
