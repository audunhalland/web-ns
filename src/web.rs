//!
//! Dynamic, object-safe web namespace.
//!

///
/// An object-safe web namespace.
///
pub trait WebNamespace: crate::TagByLocalName<Tag> {
    /// The name of this webspace.
    fn name(&self) -> &'static str;
}

///
/// Trait for accessing the origin namespace of some entity.
///
pub trait OriginWebNamespace {
    fn origin_web_namespace(&self) -> &'static dyn WebNamespace;
}

///
/// A tag with runtime polymorphism, supporting potentially different namespaces.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Tag {
    Html5(super::html5::HtmlTag),
}

impl OriginWebNamespace for Tag {
    fn origin_web_namespace(&self) -> &'static dyn WebNamespace {
        match self {
            Self::Html5(_) => &super::html5::HTML5_NS,
        }
    }
}

impl super::LocalName for Tag {
    fn local_name(&self) -> &str {
        match self {
            Self::Html5(tag) => tag.local_name(),
        }
    }
}

impl super::AttrByLocalName<Attr> for Tag {
    fn attr_by_local_name(&self, local_name: &str) -> Result<Attr, crate::Error> {
        match self {
            Self::Html5(tag) => tag.attr_by_local_name(local_name),
        }
    }
}

impl super::AttrByProperty<Attr> for Tag {
    fn attr_by_property(&self, property: &str) -> Result<Attr, crate::Error> {
        match self {
            Self::Html5(tag) => tag.attr_by_property(property),
        }
    }
}

///
/// A attribute with runtime polymorphism, supporting potentially different namespaces.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Attr {
    Html5(crate::html5::HtmlAttr),
}

impl OriginWebNamespace for Attr {
    fn origin_web_namespace(&self) -> &'static dyn WebNamespace {
        match self {
            Self::Html5(_) => &super::html5::HTML5_NS,
        }
    }
}

impl crate::LocalName for Attr {
    fn local_name(&self) -> &str {
        match self {
            Self::Html5(attr) => attr.local_name(),
        }
    }
}

impl crate::PropertyName for Attr {
    fn property_name(&self) -> &str {
        match self {
            Self::Html5(attr) => attr.property_name(),
        }
    }
}

impl crate::attr::Attribute for Attr {
    fn attr_type(&self) -> crate::attr_type::AttrType {
        match self {
            Self::Html5(attr) => attr.attr_type(),
        }
    }
}
