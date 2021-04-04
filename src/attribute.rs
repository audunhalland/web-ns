use super::Namespace;

///
/// An attribute definition.
///
pub struct Attribute(Storage);

impl Attribute {
    ///
    /// Create a new static attribute, which implies no memory allocations.
    ///
    pub const fn new_static(static_attr: &'static StaticAttribute) -> Self {
        Self(Storage::Static(static_attr))
    }

    pub fn new_dynamic(dynamic_attr: Box<dyn AttributeClass>) -> Self {
        Self(Storage::Dynamic(dynamic_attr))
    }

    pub fn instance(&self) -> (&dyn AttributeClass, Option<usize>) {
        match &self.0 {
            Storage::Static(static_attr) => (static_attr.class, Some(static_attr.static_id)),
            Storage::Dynamic(dynamic_attr) => (dynamic_attr.as_ref(), None),
        }
    }

    pub fn namespace(&self) -> &'static dyn Namespace {
        let (class, _) = self.instance();
        class.namespace()
    }

    ///
    /// Access the local name of the attribute.
    ///
    pub fn local_name(&self) -> &str {
        let (class, id) = self.instance();
        class.local_name(id)
    }
}

enum Storage {
    Static(&'static StaticAttribute),
    Dynamic(Box<dyn AttributeClass>),
}

///
/// Statically defined attribute.
/// [Namespace] implementations may represent its known attributes using no memory allocations,
/// and this type enables that.
///
pub struct StaticAttribute {
    pub class: &'static dyn AttributeClass,
    pub static_id: usize,
}

pub trait AttributeClass: Send + Sync {
    fn namespace(&self) -> &'static dyn Namespace;
    fn local_name(&self, static_id: Option<usize>) -> &str;
}
