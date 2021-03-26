pub mod attrs {
    //!
    //! Collection of HTML5 attributes.
    //!

    include!(concat!(env!("OUT_DIR"), "/codegen_html_attrs.rs"));

    pub(crate) fn internal_attr_by_name<'a>(attribute: &'a str) -> Option<&'static InternalAttr> {
        ATTRIBUTE_UNICASE_PHF
            .get(&unicase::UniCase::ascii(attribute))
            .map(|doubleref| *doubleref)
    }

    pub(crate) fn internal_attr_by_property(
        property: &str,
    ) -> Option<&'static crate::attr::attr_impl::InternalAttr> {
        PROPERTY_PHF.get(property).map(|doubleref| *doubleref)
    }
}
