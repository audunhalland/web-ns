pub mod attrs {
    include!(concat!(env!("OUT_DIR"), "/codegen_html_attrs.rs"));

    pub fn internal_attr_by_name<'a>(attribute: &'a str) -> Option<&'static InternalAttr> {
        let index = phf_unicase_lookup(&ATTRIBUTE_UNICASE_PHF, attribute)?;
        Some(&INTERNAL_ATTRS[index])
    }

    pub fn internal_attr_by_property(
        property: &str,
    ) -> Option<&'static crate::attr::attr_impl::InternalAttr> {
        let index = phf_str_lookup(&PROPERTY_PHF, property)?;
        Some(&INTERNAL_ATTRS[index])
    }
}
