use downcast_rs::{impl_downcast, Downcast};

use super::Namespace;

pub enum Name {
    Static(&'static StaticName),
    Dynamic(Box<dyn NameClass>),
}

///
/// Statically defined attribute.
/// [Namespace] implementations may represent its known attributes using no memory allocations,
/// and this type enables that.
///
pub struct StaticName {
    pub class: &'static dyn NameClass,
    pub static_id: usize,
}

pub trait NameClass: Send + Sync + Downcast {
    fn namespace(&self) -> &'static dyn Namespace;
    fn local_name(&self, static_id: Option<usize>) -> &str;

    fn equals(
        &self,
        static_id: Option<usize>,
        other_class: &dyn NameClass,
        other_static_id: Option<usize>,
    ) -> bool;

    fn dyn_hash(&self, static_id: Option<usize>, state: &mut dyn std::hash::Hasher);
}

impl_downcast!(NameClass);
