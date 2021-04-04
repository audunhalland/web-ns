use doml::attribute::{Attribute, StaticAttribute};

use crate::attr::attr_type::AttrType;
use crate::static_unicase::StaticUniCase;

pub(crate) struct StaticWebAttr {
    pub name: &'static str,
    pub property: &'static str,
    pub attr_type: AttrType,
}

pub(crate) struct StaticWebAttrClass {
    pub namespace: &'static dyn doml::Namespace,
    pub web_attrs: &'static [StaticWebAttr],
    pub attribute_unicase_map: phf::Map<StaticUniCase, usize>,
    pub property_map: phf::Map<&'static str, usize>,
}

impl StaticWebAttrClass {
    pub fn attribute_by_local_name(
        &self,
        local_name: &str,
        static_attrs: &'static [StaticAttribute],
    ) -> Result<Attribute, doml::Error> {
        self.attribute_unicase_map
            .get(&unicase::UniCase::ascii(local_name))
            .map(|static_id| Attribute::new_static(&static_attrs[*static_id]))
            .ok_or_else(|| doml::Error::InvalidAttribute)
    }
}

impl doml::attribute::AttributeClass for StaticWebAttrClass {
    fn namespace(&self) -> &'static dyn doml::Namespace {
        self.namespace
    }

    fn local_name(&self, static_id: Option<usize>) -> &str {
        self.web_attrs[static_id.unwrap()].name
    }

    fn metadata<'a>(&'a self, static_id: Option<usize>, key: &str) -> Option<&'a str> {
        match key {
            crate::metadata::PROPERTY => Some(self.web_attrs[static_id.unwrap()].property),
            _ => None,
        }
    }
}
