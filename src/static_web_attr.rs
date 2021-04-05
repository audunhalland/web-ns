use dyn_symbol::Symbol;

use crate::new::Attribute;
use crate::Error;

use crate::attr::attr_type::AttrType;
use crate::static_unicase::StaticUniCase;

pub(crate) struct StaticWebAttr {
    pub name: &'static str,
    pub property: &'static str,
    pub attr_type: AttrType,
}

pub(crate) struct StaticWebAttrNS<const NS: u8> {
    pub web_ns: crate::WebNS,
    pub web_attrs: &'static [StaticWebAttr],
    pub attribute_unicase_map: phf::Map<StaticUniCase, u32>,
    pub property_map: phf::Map<&'static str, u32>,
}

impl<const NS: u8> StaticWebAttrNS<NS> {
    pub fn attribute_by_local_name(&'static self, local_name: &str) -> Result<Attribute, Error> {
        self.attribute_unicase_map
            .get(&unicase::UniCase::ascii(local_name))
            .map(|static_id| Attribute(Symbol::Static(self, *static_id)))
            .ok_or_else(|| Error::InvalidAttribute)
    }

    pub fn attribute_by_property_name(
        &'static self,
        property_name: &str,
    ) -> Result<Attribute, Error> {
        self.property_map
            .get(property_name)
            .map(|static_id| Attribute(Symbol::Static(self, *static_id)))
            .ok_or_else(|| Error::InvalidAttribute)
    }

    pub fn property_name(&self, static_id: u32) -> &str {
        self.web_attrs[static_id as usize].property
    }
}

impl<const NS: u8> dyn_symbol::namespace::Static for StaticWebAttrNS<NS> {
    fn namespace_name(&self) -> &str {
        self.web_ns.name()
    }

    fn symbol_name(&self, id: u32) -> &str {
        self.web_attrs[id as usize].name
    }
}
