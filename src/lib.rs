//! Web namespaces.
//!
//! # Usage
//!
//! Enum definitions for the tags and attributes used in web standards:
//! ```
//! use web_ns::*;
//! use web_ns::web::OriginWebNamespace;
//!
//! let div: web::Tag = html5::HTML5_NS.tag_by_local_name("DIV").unwrap();
//! assert_eq!(div, web::Tag::Html5(html5::HtmlTag::Div));
//! assert_eq!(div.local_name(), "div");
//! assert_eq!(div.is_void(), false);
//! assert_eq!(div.origin_web_namespace().name(), "html5");
//!
//! let class: web::Attr = div.attr_by_local_name("CLASS").unwrap();
//! assert_eq!(class, web::Attr::Html5(html5::HtmlAttr::Class));
//! assert_eq!(class.local_name(), "class");
//! assert_eq!(class.property_name(), "className");
//! assert_eq!(class, div.attr_by_property("className").unwrap());
//! ```
//!
//! The [crate::web] module has support for dynamically changing namespaces:
//! ```
//! use web_ns::*;
//!
//! let svg: web::Tag = html5::HTML5_NS.tag_by_local_name("SVG").unwrap();
//! assert_eq!(svg, web::Tag::Svg(svg::SvgTag::Svg));
//! ```
//!

#![forbid(unsafe_code)]

pub mod attr;
pub mod web;

pub mod html5;
pub mod svg;

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
    ///
    /// Check if something is void.
    ///
    /// Example:
    /// ```
    /// use web_ns::IsVoid;
    ///
    /// assert!(web_ns::html5::HtmlTag::Img.is_void());
    /// assert!(!web_ns::html5::HtmlTag::B.is_void());
    /// ```
    fn is_void(&self) -> bool;
}

///
/// Tag lookup by the tag's local name.
/// name being local means that this trait should be implemented by namespaces.
///
/// The trait is generic so that tags can be looked up with different "granularity":
/// There needs to be an object-safe way to look up tags where the type of that tag
/// is runtime encoded ([web::Tag]).
///
pub trait TagByLocalName<T> {
    ///
    /// Do tag lookup by local name.
    ///
    /// Example:
    /// ```
    /// use web_ns::*;
    ///
    /// // "specific" version:
    /// let div: html5::HtmlTag = html5::HTML5_NS.tag_by_local_name("DIV").unwrap();
    /// assert_eq!(div, html5::HtmlTag::Div);
    ///
    /// // "generic" version:
    /// let div: web::Tag = html5::HTML5_NS.tag_by_local_name("DIV").unwrap();
    /// assert_eq!(div, web::Tag::Html5(html5::HtmlTag::Div));
    /// ```
    ///
    fn tag_by_local_name(&self, local_name: &str) -> Result<T, Error>;
}

///
/// Attribute lookup by the attribute's local name.
/// Attribute types can theoretically vary from element to element, so this trait
/// should be implemented by
/// name being local means that this trait should be implemented by namespaces.
///
/// The trait is generic so that tags can be looked up with different "granularity":
/// There needs to be an object-safe way to look up tags where the origin namespace of that tag
/// is runtime encoded ([web::Tag]).
///
pub trait AttrByLocalName<A> {
    ///
    ///
    ///
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
