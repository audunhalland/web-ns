use super::{Namespace, Static};

///
/// An element definition.
///
pub struct Element(Storage);

impl Element {
    pub const fn new_static(static_element: &'static StaticElement) -> Self {
        Self(Storage::Static(static_element))
    }

    pub fn new_dynamic(dynamic_element: Box<dyn DynamicElement>) -> Self {
        Self(Storage::Dynamic(dynamic_element))
    }

    pub fn namespace(&self) -> &'static dyn Namespace {
        match &self.0 {
            Storage::Static(static_element) => static_element.namespace,
            Storage::Dynamic(dynamic_element) => dynamic_element.namespace(),
        }
    }

    ///
    /// Access the local name of the attribute.
    ///
    pub fn local_name(&self) -> &str {
        match &self.0 {
            Storage::Static(static_element) => static_element
                .namespace
                .get_static_local_name(Static::Element(static_element.static_id)),
            Storage::Dynamic(dynamic_element) => dynamic_element.local_name(),
        }
    }
}

enum Storage {
    Static(&'static StaticElement),
    Dynamic(Box<dyn DynamicElement>),
}

pub struct StaticElement {
    pub namespace: &'static dyn Namespace,
    pub static_id: usize,
}

pub trait DynamicElement: std::any::Any {
    fn namespace(&self) -> &'static dyn Namespace;
    fn local_name(&self) -> &str;
}
