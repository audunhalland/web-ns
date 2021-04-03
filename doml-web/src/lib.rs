#![forbid(unsafe_code)]

use doml::attribute::Attribute as DomlAttribute;
use doml::element::Element as DomlElement;

pub mod attr;

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

pub struct Html5(Private);

pub const HTML5: Html5 = Html5(Private);

impl doml::Namespace for Html5 {
    fn element_by_local_name(&self, local_name: &str) -> Result<DomlElement, doml::Error> {
        todo!()
    }

    fn attribute_by_local_name(
        &self,
        element: &DomlElement,
        local_name: &str,
    ) -> Result<DomlAttribute, doml::Error> {
        todo!()
    }

    fn get_static_local_name(&self, input: doml::Static) -> &'static str {
        todo!("lookup")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
