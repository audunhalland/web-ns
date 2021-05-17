//!
//! Dynamic, object-safe web namespace.
//!

use super::Error;

///
/// An object-safe web namespace.
///
pub trait WebNamespace {
    /// The name of this webspace.
    fn name(&self) -> &'static str;

    ///
    /// Look up a tag name within this namespace.
    /// Tag names correspond to elements in a DOM model.
    ///
    fn tag_by_local_name(&self, local_name: &str) -> Result<Tag, Error>;

    ///
    /// Look up an attribute by its local name.
    ///
    fn attribute_by_local_name(&self, tag: &Tag, local_name: &str) -> Result<Attr, Error>;
}

///
/// Trait for accessing the origin namespace of some entity.
///
trait OriginWebNamespace {
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

///
/// A attribute with runtime polymorphism, supporting potentially different namespaces.
///
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Attr {
    Html5(super::html5::HtmlAttr),
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
