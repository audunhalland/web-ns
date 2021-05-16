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

impl crate::PropertyName for DataAttr {
    fn property_name(&self) -> &str {
        &self.strbuf[self.buf_property_start..]
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
