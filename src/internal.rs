use crate::attr_type::AttrFormat;
use crate::attr_type::AttrType;

pub struct InternalAttr {
    pub attribute: &'static str,
    pub property: &'static str,
    pub format: AttrFormat,
    pub attr_type: AttrType,
}

impl PartialEq<InternalAttr> for InternalAttr {
    fn eq(&self, other: &Self) -> bool {
        self.attribute == other.attribute
    }
}
impl Eq for InternalAttr {}

impl std::hash::Hash for InternalAttr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.attribute.hash(state);
    }
}

impl PartialOrd for InternalAttr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.attribute.partial_cmp(other.attribute)
    }
}

impl Ord for InternalAttr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attribute.cmp(other.attribute)
    }
}
