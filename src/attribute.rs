use super::name::*;
use super::Namespace;

///
/// An attribute definition.
///
pub struct Attribute(Name);

impl Attribute {
    ///
    /// Create a new static attribute, which implies no memory allocations.
    ///
    pub const fn new_static(name: &'static StaticName) -> Self {
        Self(Name::Static(name))
    }

    pub fn new_dynamic(name: Box<dyn NameClass>) -> Self {
        Self(Name::Dynamic(name))
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
    pub fn instance(&self) -> (&dyn NameClass, Option<usize>) {
        match &self.0 {
            Name::Static(name) => (name.class, Some(name.static_id)),
            Name::Dynamic(name) => (name.as_ref(), None),
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
        class_a.equals(id_a, class_b, id_b)
    }
}

impl Eq for Attribute {}

impl std::hash::Hash for Attribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (class, id) = self.instance();
        class.dyn_hash(id, state);
    }
}
