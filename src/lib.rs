//! Web namespaces.
//!
//! # Usage
//!
//! ```
//! use web_ns::*;
//!
//! let element = html5::HTML5_NS.element_by_local_name("div").unwrap();
//! assert_eq!(element, html5::tags::HtmlTag::Div);
//! ```
//!

#![forbid(unsafe_code)]

pub mod attr;
pub mod web;

pub mod html5;

mod static_unicase;

pub use attr::*;

struct Private;

///
/// Trait for anything that has a local/unqualified name.
///
pub trait LocalName {
    fn local_name(&self) -> &str;
}

///
/// Trait for anything that has a JS property name.
///
pub trait PropertyName {
    fn property_name(&self) -> &str;
}

///
/// Trait for accessing "voidness" of tag names.
/// a tag being "Void" means that it must be self closing:
/// `<img />`
/// vs. non-void that cannot be self-closing:
/// `<b></b>`
///
pub trait IsVoid {
    fn is_void(&self) -> bool;
}

///
/// A typed namespace, with a complete type system for tags an attributes within the namespace.
///
pub trait TypedNamespace {
    type Tag;
    type Attribute;

    ///
    /// Look up a tag name within this namespace.
    /// Tag names correspond to elements in a DOM model.
    ///
    fn typed_tag_by_local_name(&self, local_name: &str) -> Result<Self::Tag, Error>;

    ///
    /// Look up an attribute by its local name.
    ///
    fn typed_attribute_by_local_name(
        &self,
        tag: &Self::Tag,
        local_name: &str,
    ) -> Result<Self::Attribute, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    InvalidAttributeValue,
    NamespaceMismatch,
}
