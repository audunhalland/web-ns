//! SVG implementation

use crate::Error;

use super::*;

mod tags {
    //! Tag definitions for SVG
    include!(concat!(env!("OUT_DIR"), "/codegen_svg_tag_enum.rs"));
}

mod attributes {
    //! Attribute definitions for HTML5
    include!(concat!(env!("OUT_DIR"), "/codegen_svg_attr_enum.rs"));
}

pub use attributes::SvgAttr;
pub use tags::SvgTag;

/// A [web::WebNamespace] implementation for SVG.
pub struct SvgNamespace(Private);

/// The global [SvgNamespace] instance.
pub const SVG_NS: SvgNamespace = SvgNamespace(Private);

impl super::web::WebNamespace for SvgNamespace {
    fn name(&self) -> &'static str {
        "svg"
    }
}

impl crate::TagByLocalName<tags::SvgTag> for SvgNamespace {
    fn tag_by_local_name(&self, local_name: &str) -> Result<tags::SvgTag, Error> {
        tags::STATIC_LOCAL_NAME_LOOKUP
            .get(&unicase::UniCase::ascii(local_name))
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

impl crate::TagByLocalName<crate::web::Tag> for SvgNamespace {
    fn tag_by_local_name(&self, local_name: &str) -> Result<crate::web::Tag, Error> {
        self.tag_by_local_name(local_name)
            .map(|tag| super::web::Tag::Svg(tag))
    }
}

impl crate::AttrByLocalName<attributes::SvgAttr> for tags::SvgTag {
    fn attr_by_local_name(&self, local_name: &str) -> Result<attributes::SvgAttr, Error> {
        attributes::STATIC_LOCAL_NAME_LOOKUP
            .get(&unicase::UniCase::ascii(local_name))
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

impl crate::AttrByProperty<attributes::SvgAttr> for tags::SvgTag {
    fn attr_by_property(&self, property: &str) -> Result<attributes::SvgAttr, Error> {
        attributes::STATIC_PROPERTY_LOOKUP
            .get(property)
            .map(Clone::clone)
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

impl crate::AttrByLocalName<crate::web::Attr> for tags::SvgTag {
    fn attr_by_local_name(&self, local_name: &str) -> Result<crate::web::Attr, Error> {
        self.attr_by_local_name(local_name)
            .map(|attr| super::web::Attr::Svg(attr))
    }
}

impl crate::AttrByProperty<crate::web::Attr> for tags::SvgTag {
    fn attr_by_property(&self, property: &str) -> Result<crate::web::Attr, Error> {
        self.attr_by_property(property)
            .map(|attr| super::web::Attr::Svg(attr))
    }
}
