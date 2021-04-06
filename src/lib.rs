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

#[derive(Clone, Copy, Eq, PartialEq)]
enum WebNS {
    HTML5 = 0,
}

impl WebNS {
    fn name(&self) -> &'static str {
        match self {
            Self::HTML5 => "html5",
        }
    }

    fn web_namespace(&self) -> &'static dyn WebNamespace {
        match self {
            Self::HTML5 => &html5::HTML5_NS,
        }
    }
}

struct Private;

pub trait WebNamespace {
    fn element_by_local_name(&self, _local_name: &str) -> Result<Symbol, Error>;

    fn attribute_by_local_name(&self, _: &Symbol, local_name: &str) -> Result<Symbol, Error>;

    fn attribute_by_property(&self, property_name: &str) -> Result<Symbol, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    InvalidAttributeValue,
}
