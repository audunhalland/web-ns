use super::attr_type::AttrType;
use super::*;
use dyn_symbol::namespace::Dynamic;

#[derive(Clone)]
pub struct DataAttr {
    pub strbuf: String,
    pub buf_property_start: usize,
    pub attr_type: AttrType,
}

impl DataAttr {
    pub fn attribute_info(&self) -> AttributeInfo {
        AttributeInfo {
            web_ns: WebNS::HTML5,
            attr_type: self.attr_type.clone(),
            property: Some(self.property_name()),
        }
    }

    pub fn attr_name(&self) -> &str {
        &self.strbuf[..self.buf_property_start]
    }

    pub fn property_name(&self) -> &str {
        &self.strbuf[self.buf_property_start..]
    }
}

impl crate::LocalName for DataAttr {
    fn local_name(&self) -> &str {
        &self.strbuf[..self.buf_property_start]
    }
}

impl crate::attr::Attribute for DataAttr {
    fn attr_type(&self) -> crate::attr::attr_type::AttrType {
        self.attr_type
    }
}

impl crate::PropertyName for DataAttr {
    fn property_name(&self) -> &str {
        &self.strbuf[self.buf_property_start..]
    }
}

impl PartialEq for DataAttr {
    fn eq(&self, rhs: &Self) -> bool {
        use crate::LocalName;
        self.local_name() == rhs.local_name()
    }
}

impl Eq for DataAttr {}

impl Ord for DataAttr {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        use crate::LocalName;
        self.local_name().cmp(rhs.local_name())
    }
}

impl PartialOrd for DataAttr {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl std::hash::Hash for DataAttr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use crate::LocalName;
        self.local_name().hash(state);
    }
}

impl std::fmt::Debug for DataAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::LocalName;
        write!(f, "{}", self.local_name())
    }
}

impl Dynamic for DataAttr {
    fn namespace_name(&self) -> &str {
        "html5::data"
    }

    fn symbol_name(&self) -> &str {
        &self.strbuf[..self.buf_property_start]
    }

    fn dyn_clone(&self) -> Box<dyn Dynamic> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, rhs: &dyn Dynamic) -> bool {
        self.attr_name() == rhs.downcast_ref::<Self>().unwrap().attr_name()
    }

    fn dyn_cmp(&self, rhs: &dyn Dynamic) -> std::cmp::Ordering {
        let rhs_attr_name = rhs.downcast_ref::<Self>().unwrap().attr_name();
        self.attr_name().cmp(rhs_attr_name)
    }

    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
        state.write(self.attr_name().as_bytes());
        state.write_u8(0xff)
    }
}
