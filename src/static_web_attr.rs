use crate::{attr, Error, WebNS};

use crate::attr::attr_type::AttrType;
use crate::static_unicase::StaticUniCase;

pub(crate) struct StaticWebAttr {
    pub web_ns: WebNS,
    pub name: &'static str,
    pub property: &'static str,
    pub attr_type: AttrType,
}

pub(crate) struct StaticWebAttrLookupTables {
    pub attribute_unicase_map: phf::Map<StaticUniCase, u32>,
    pub property_map: phf::Map<&'static str, u32>,
}

impl StaticWebAttrLookupTables {
    pub fn attribute_by_local_name(&'static self, local_name: &str) -> Result<u32, Error> {
        self.attribute_unicase_map
            .get(&unicase::UniCase::ascii(local_name))
            .cloned()
            .ok_or_else(|| Error::InvalidAttribute)
    }

    pub fn attribute_by_property_name(&'static self, property_name: &str) -> Result<u32, Error> {
        self.property_map
            .get(property_name)
            .cloned()
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

pub(crate) struct StaticWebAttrSymbolNamespace {
    pub web_attrs: &'static [StaticWebAttr],
}

impl StaticWebAttrSymbolNamespace {
    pub fn attribte_info(&self, static_id: u32) -> attr::AttributeInfo {
        let attr = &self.web_attrs[static_id as usize];
        attr::AttributeInfo {
            web_ns: attr.web_ns.clone(),
            attr_type: attr.attr_type.clone(),
            property: Some(attr.property),
        }
    }

    pub fn property_name(&self, static_id: u32) -> &str {
        self.web_attrs[static_id as usize].property
    }
}

impl dyn_symbol::namespace::Static for StaticWebAttrSymbolNamespace {
    fn namespace_name(&self) -> &str {
        "web-attr"
    }

    fn symbol_name(&self, id: u32) -> &str {
        self.web_attrs[id as usize].name
    }
}
