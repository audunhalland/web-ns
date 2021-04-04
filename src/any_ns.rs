use super::attribute::*;
use super::element::*;
use super::{Error, Namespace};

///
/// A completely untyped namespace, which can be used for representing any kind of markup or DOM,
/// with no validation. Note that this is more expensive to use than just using strings for types.
///
pub struct AnyNamespace(Private);

struct Private;

pub static ANY_NS: AnyNamespace = AnyNamespace(Private);

impl super::Namespace for AnyNamespace {
    fn element_by_local_name(&self, local_name: &str) -> Result<Element, Error> {
        Ok(Element::new_dynamic(Box::new(AnyElement {
            local_name: local_name.to_string(),
        })))
    }

    fn attribute_by_local_name(
        &self,
        _element: &Element,
        local_name: &str,
    ) -> Result<Attribute, Error> {
        Ok(Attribute::new_dynamic(Box::new(AnyAttribute {
            local_name: local_name.to_string(),
        })))
    }
}

struct AnyElement {
    local_name: String,
}

impl ElementClass for AnyElement {
    fn namespace(&self) -> &'static dyn Namespace {
        &ANY_NS
    }

    fn local_name(&self, _: Option<usize>) -> &str {
        &self.local_name
    }
}

struct AnyAttribute {
    local_name: String,
}

impl AttributeClass for AnyAttribute {
    fn namespace(&self) -> &'static dyn Namespace {
        &ANY_NS
    }

    fn local_name(&self, _: Option<usize>) -> &str {
        &self.local_name
    }

    fn metadata<'a>(&'a self, _: Option<usize>, _: &str) -> Option<&'a str> {
        None
    }
}
