use super::attr_type::AttrType;

#[derive(Clone)]
pub enum AttrImpl {
    Internal(&'static InternalAttr),
    Data(Box<DataAttr>),
}
pub struct InternalAttr {
    pub attribute: &'static str,
    pub property: &'static str,
    pub attr_type: AttrType,
}

#[derive(Clone)]
pub struct DataAttr {
    pub strbuf: String,
    pub buf_property_start: usize,
    pub attr_type: AttrType,
}
