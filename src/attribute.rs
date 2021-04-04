use downcast_rs::{impl_downcast, Downcast};

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

    ///
    /// Access the originating namespace of the attribute.
    ///
    pub fn namespace(&self) -> &'static dyn Namespace {
        self.instance().0.namespace()
    }

    ///
    /// Access the local name of the attribute.
    ///
    pub fn local_name(&self) -> &str {
        let (class, id) = self.instance();
        class.local_name(id)
    }

    #[inline]
    pub fn instance(&self) -> (&dyn AttributeClass, Option<usize>) {
        match &self.0 {
            Storage::Static(static_attr) => (static_attr.class, Some(static_attr.static_id)),
            Storage::Dynamic(dynamic_attr) => (dynamic_attr.as_ref(), None),
        }
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.local_name())
    }
}

impl std::cmp::PartialEq<Attribute> for Attribute {
    fn eq(&self, rhs: &Attribute) -> bool {
        let (class_a, id_a) = self.instance();
        let (class_b, id_b) = rhs.instance();
        class_a.eq(id_a, class_b, id_b)
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

impl PartialEq<StaticAttribute> for StaticAttribute {
    fn eq(&self, rhs: &StaticAttribute) -> bool {
        std::ptr::eq(self.class, rhs.class) && self.static_id == rhs.static_id
    }
}

pub trait AttributeClass: Send + Sync + Downcast {
    fn namespace(&self) -> &'static dyn Namespace;
    fn eq(
        &self,
        static_id: Option<usize>,
        other_class: &dyn AttributeClass,
        other_static_id: Option<usize>,
    ) -> bool;
    fn local_name(&self, static_id: Option<usize>) -> &str;
}

impl_downcast!(AttributeClass);
