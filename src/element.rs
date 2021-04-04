use super::Namespace;

///
/// An element definition.
///
pub struct Element(Storage);

impl Element {
    pub const fn new_static(static_element: &'static StaticElement) -> Self {
        Self(Storage::Static(static_element))
    }

    pub fn new_dynamic(dynamic_element: Box<dyn ElementClass>) -> Self {
        Self(Storage::Dynamic(dynamic_element))
    }

    pub fn instance(&self) -> (&dyn ElementClass, Option<usize>) {
        match &self.0 {
            Storage::Static(static_element) => {
                (static_element.class, Some(static_element.static_id))
            }
            Storage::Dynamic(dynamic_element) => (dynamic_element.as_ref(), None),
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
    Static(&'static StaticElement),
    Dynamic(Box<dyn ElementClass>),
}

pub struct StaticElement {
    pub class: &'static dyn ElementClass,
    pub static_id: usize,
}

pub trait ElementClass: Send + Sync {
    fn namespace(&self) -> &'static dyn Namespace;
    fn local_name(&self, static_id: Option<usize>) -> &str;
}
