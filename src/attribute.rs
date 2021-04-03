use super::Namespace;

///
/// An attribute definition.
///
pub struct Attribute(Storage);

impl Attribute {
    ///
    /// Create a new static attribute, which implies no memory allocations.
    ///
    /// # Example
    /// ```
    /// use doml::any_ns::ANY_NS;
    /// use doml::attribute::*;
    ///
    /// static ATTR: StaticAttribute = StaticAttribute {
    ///     namespace: &ANY_NS,
    ///     local_name: "test",
    /// };
    ///
    /// let attr = Attribute::new_static(&ATTR);
    /// assert_eq!(attr.local_name(), "test");
    /// ```
    ///
    pub const fn new_static(static_attr: &'static StaticAttribute) -> Self {
        Self(Storage::Static(static_attr))
    }

    pub fn new_dynamic(dynamic_attr: Box<dyn DynamicAttribute>) -> Self {
        Self(Storage::Dynamic(dynamic_attr))
    }

    pub fn namespace(&self) -> &'static dyn Namespace {
        match &self.0 {
            Storage::Static(static_attr) => static_attr.namespace,
            Storage::Dynamic(dynamic_attr) => dynamic_attr.namespace(),
        }
    }

    ///
    /// Access the local name of the attribute.
    ///
    pub fn local_name(&self) -> &str {
        match &self.0 {
            Storage::Static(static_attr) => static_attr.local_name,
            Storage::Dynamic(dynamic_attr) => dynamic_attr.local_name(),
        }
    }
}

enum Storage {
    Static(&'static StaticAttribute),
    Dynamic(Box<dyn DynamicAttribute>),
}

///
/// Statically defined attribute.
/// [Namespace] implementations may represent its known attributes using no memory allocations,
/// and this type enables that.
///
pub struct StaticAttribute {
    pub namespace: &'static dyn Namespace,
    pub local_name: &'static str,
}

///
/// Dynamically defined attribute.
/// A [Namespace] may have a non-fixed set of allowed attributes, and in that case the name
/// of the attribute must be dynamically allocated.
///
pub trait DynamicAttribute {
    fn namespace(&self) -> &'static dyn Namespace;
    fn local_name(&self) -> &str;
}
