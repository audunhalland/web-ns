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

pub trait TagByLocalName<T> {
    fn tag_by_local_name(&self, local_name: &str) -> Result<T, Error>;
}

pub trait AttrByLocalName<A> {
    fn attr_by_local_name(&self, local_name: &str) -> Result<A, Error>;
}

pub trait AttrByProperty<A> {
    fn attr_by_property(&self, property: &str) -> Result<A, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    InvalidAttributeValue,
    NamespaceMismatch,
}
