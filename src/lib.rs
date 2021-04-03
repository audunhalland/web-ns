//!
//! `doml` is a utility crate for modelling typed/validated Document Object Models and corresponding markup languages.
//!
//! It does not model any DOM itself, virtual nor concrete, but provides data types
//! to represent the primitive building blocks of such data structures efficiently in memory:
//!
//! * [element::Element]
//! * [attribute::Attribute]
//!
//! `doml` exports the [Namespace] trait, which can be implemented for a specific
//! DOM model/schema (e.g. the HTML spec, or some XML schema). Namespaces may defined statically,
//! and the returned types may be configured to use a minimal amount of memory with no allocation
//! for statically known elements or attributes.
//!
//! Elements and attributes refer back to their originating namespace, so a document may be modelled
//! using a mix of namespaces.
//!

pub mod any_ns;
pub mod attribute;
pub mod element;

///
/// Abstract markup language namespace.
///
pub trait Namespace: Send + Sync + std::any::Any {
    ///
    /// Look up an element by its local name within the namespace.
    ///
    /// # Example
    ///
    /// ```
    /// use doml::Namespace;
    /// use doml::any_ns::ANY_NS;
    ///
    /// let element = ANY_NS.element_by_local_name("foo").unwrap();
    /// assert_eq!(element.local_name(), "foo");
    /// ```
    fn element_by_local_name(&self, local_name: &str) -> Result<element::Element, Error>;

    ///
    /// Look up an attribute by its local name within the namespace, given its containing element.
    ///
    /// # Example
    ///
    /// ```
    /// use doml::Namespace;
    /// use doml::any_ns::ANY_NS;
    ///
    /// let element = ANY_NS.element_by_local_name("foo").unwrap();
    /// let attribute = ANY_NS.attribute_by_local_name(&element, "bar").unwrap();
    /// assert_eq!(attribute.local_name(), "bar");
    /// ```
    fn attribute_by_local_name(
        &self,
        element: &element::Element,
        local_name: &str,
    ) -> Result<attribute::Attribute, Error>;
}

#[derive(Debug)]
pub enum Error {
    InvalidNamespace,
    InvalidAttribute,
}
