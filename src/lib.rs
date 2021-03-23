
mod macros;
mod internal;

mod html;

pub enum Schema {
    Html5
}

pub enum AttributeValue {
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Attribute(AttrAlloc);

///
/// A web attribute.
///
impl Attribute {
    ///
    /// Access the markup attribute name of this attribute.
    ///
    pub fn attribute(&self) -> &str {
        match &self.0 {
            AttrAlloc::Internal(attr) => attr.attribute,
            AttrAlloc::Data(attr) => &attr.strbuf[..attr.buf_property_start],
        }
    }

    ///
    /// Access the JS property name of this attribute.
    ///
    pub fn property(&self) -> &str {
        match &self.0 {
            AttrAlloc::Internal(attr) => attr.property,
            AttrAlloc::Data(attr) => &attr.strbuf[attr.buf_property_start..],
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum AttrAlloc {
    Internal(&'static internal::InternalAttr),
    Data(Box<DataAttr>),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct DataAttr {
    strbuf: String,
    buf_property_start: usize,
}

pub fn parse_attribute(attribute: &str, schema: Schema) -> Option<Attribute> {
    match html::attrs::internal_attr_by_name(attribute) {
        Some(internal_attr) => Some(Attribute(AttrAlloc::Internal(internal_attr))),
        None => None
    }
}

pub fn parse_property(property: &str, schema: Schema) -> Option<Attribute> {
    match html::attrs::internal_attr_by_property(property) {
        Some(internal_attr) => Some(Attribute(AttrAlloc::Internal(internal_attr))),
        None => None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
