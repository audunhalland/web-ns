use std::hash::Hasher;

use doml::attribute::Attribute;
use doml::name::{NameClass, StaticName};

use crate::attr::attr_type::AttrType;
use crate::static_unicase::StaticUniCase;

pub(crate) struct StaticWebAttr {
    pub name: &'static str,
    pub property: &'static str,
    pub attr_type: AttrType,
}

pub(crate) struct StaticWebAttrClass {
    pub web_ns: crate::WebNS,
    pub namespace: &'static dyn doml::Namespace,
    pub web_attrs: &'static [StaticWebAttr],
    pub attribute_unicase_map: phf::Map<StaticUniCase, usize>,
    pub property_map: phf::Map<&'static str, usize>,
}

impl StaticWebAttrClass {
    pub fn attribute_by_local_name(
        &self,
        local_name: &str,
        static_names: &'static [StaticName],
    ) -> Result<Attribute, doml::Error> {
        self.attribute_unicase_map
            .get(&unicase::UniCase::ascii(local_name))
            .map(|static_id| Attribute::new_static(&static_names[*static_id]))
            .ok_or_else(|| doml::Error::InvalidAttribute)
    }

    pub fn attribute_by_property_name(
        &self,
        property_name: &str,
        static_names: &'static [StaticName],
    ) -> Result<Attribute, doml::Error> {
        self.property_map
            .get(property_name)
            .map(|static_id| Attribute::new_static(&static_names[*static_id]))
            .ok_or_else(|| doml::Error::InvalidAttribute)
    }

    pub fn property_name(&self, static_id: usize) -> &str {
        self.web_attrs[static_id].property
    }
}

impl NameClass for StaticWebAttrClass {
    fn namespace(&self) -> &'static dyn doml::Namespace {
        self.namespace
    }

    fn local_name(&self, static_id: Option<usize>) -> &str {
        self.web_attrs[static_id.unwrap()].name
    }

    fn equals(
        &self,
        self_id: Option<usize>,
        other_class: &dyn NameClass,
        other_id: Option<usize>,
    ) -> bool {
        if let Some(other) = other_class.downcast_ref::<StaticWebAttrClass>() {
            self.web_ns == other.web_ns && self_id == other_id
        } else {
            false
        }
    }

    fn dyn_hash(&self, static_id: Option<usize>, state: &mut dyn Hasher) {
        state.write_u8(self.web_ns as u8);
        state.write_usize(static_id.unwrap())
    }
}
