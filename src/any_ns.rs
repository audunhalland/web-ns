use std::hash::Hasher;

use super::attribute::*;
use super::element::*;
use super::name::NameClass;
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

impl NameClass for AnyAttribute {
    fn namespace(&self) -> &'static dyn Namespace {
        &ANY_NS
    }

    fn equals(&self, _: Option<usize>, other_class: &dyn NameClass, _: Option<usize>) -> bool {
        if let Some(other) = other_class.downcast_ref::<AnyAttribute>() {
            self.local_name == other.local_name
        } else {
            false
        }
    }

    fn local_name(&self, _: Option<usize>) -> &str {
        &self.local_name
    }

    fn dyn_hash(&self, _: Option<usize>, state: &mut dyn Hasher) {
        state.write(self.local_name.as_bytes());
        state.write_u8(0xff)
    }
}

#[cfg(test)]
mod tests {
    use crate::Namespace;

    use super::ANY_NS;

    #[test]
    fn attribute_eq() {
        let elem = ANY_NS.element_by_local_name("foo").unwrap();
        let a = ANY_NS.attribute_by_local_name(&elem, "bar").unwrap();
        let b = ANY_NS.attribute_by_local_name(&elem, "bar").unwrap();

        assert_eq!(a, b);
    }
}
