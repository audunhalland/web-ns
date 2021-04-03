#![forbid(unsafe_code)]

use doml::attribute::Attribute as DomlAttribute;
use doml::element::Element as DomlElement;

mod attr;

mod static_unicase;

pub mod schema {
    pub mod html5;
}

///
/// A specific document Schema used on the web.
///
pub enum Schema {
    Html5,
}

struct Private;

pub mod html5 {
    use super::*;

    mod attrs {
        include!(concat!(env!("OUT_DIR"), "/codegen_static_html_attrs.rs"));
    }

    pub struct Html5Namespace(Private);

    pub const HTML5_NS: Html5Namespace = Html5Namespace(Private);

    impl doml::Namespace for Html5Namespace {
        fn element_by_local_name(&self, local_name: &str) -> Result<DomlElement, doml::Error> {
            todo!()
        }

        fn attribute_by_local_name(
            &self,
            _element: &DomlElement,
            local_name: &str,
        ) -> Result<DomlAttribute, doml::Error> {
            attrs::ATTRIBUTE_UNICASE_PHF
                .get(&unicase::UniCase::ascii(local_name))
                .map(|web_attr| DomlAttribute::new_static(&web_attr.static_attribute))
                .ok_or_else(|| doml::Error::InvalidAttribute)
        }

        fn get_static_local_name(&self, input: doml::Static) -> &'static str {
            match input {
                doml::Static::Element(_static_id) => todo!("no support for elements yet"),
                doml::Static::Attribute(static_id) => attrs::WEB_ATTRS[static_id].name,
            }
        }
    }
}
