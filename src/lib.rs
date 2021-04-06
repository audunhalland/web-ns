//! Web namespaces.
//!
//! # Usage
//!
//! ```
//! use web_ns::*;
//!
//! let element = html5::HTML5_NS.element_by_local_name("div").unwrap();
//! ```
//!

#![forbid(unsafe_code)]

pub mod attr;

pub mod html5;

mod static_unicase;
mod static_web_attr;
mod static_web_tag;
mod symbols;

use dyn_symbol::Symbol;

pub use attr::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum WebNS {
    HTML5,
}

impl WebNS {
    fn web_namespace(&self) -> &'static dyn WebNamespace {
        match self {
            Self::HTML5 => &html5::HTML5_NS,
        }
    }
}

struct Private;

///
/// A web namespace.
///
pub trait WebNamespace {
    /// The name of this webspace.
    fn name(&self) -> &'static str;

    ///
    /// Look up an element by its local name within the namespace.
    ///
    /// The name matching is case-insensitive.
    ///
    fn element_by_local_name(&self, local_name: &str) -> Result<Symbol, Error>;

    ///
    /// Look up an attribute by its containing element and its local name within the namespace.
    ///
    /// The name matching is case-insensitive.
    ///
    fn attribute_by_local_name(&self, element: &Symbol, local_name: &str) -> Result<Symbol, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    InvalidAttributeValue,
}
