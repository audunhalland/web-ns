use super::attr_type::AttrType;
use crate::static_unicase::StaticUniCase;

#[derive(Clone)]
pub enum AttrImpl {
    Internal(&'static InternalAttr),
    Data(Box<DataAttr>),
}
pub struct InternalAttr {
    pub attribute: StaticUniCase,
    pub property: &'static str,
    pub attr_type: AttrType,
}

#[derive(Clone)]
pub struct DataAttr {
    pub strbuf: String,
    pub buf_property_start: usize,
    pub attr_type: AttrType,
}
