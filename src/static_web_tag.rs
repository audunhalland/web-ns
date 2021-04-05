use dyn_symbol::Symbol;

use crate::{Error, WebNS};

use crate::static_unicase::StaticUniCase;
use crate::symbols::tag::__TAG_SYMBOL_NS;

pub(crate) struct StaticWebTag {
    pub web_ns: WebNS,
    pub name: &'static str,
}

pub(crate) struct StaticWebTagLookupTable {
    pub tag_unicase_map: phf::Map<StaticUniCase, u32>,
}

impl StaticWebTagLookupTable {
    pub fn tag_by_local_name(&'static self, local_name: &str) -> Result<Symbol, Error> {
        self.tag_unicase_map
            .get(&unicase::UniCase::ascii(local_name))
            .map(|static_id| Symbol::Static(&__TAG_SYMBOL_NS, *static_id))
            .ok_or_else(|| Error::InvalidAttribute)
    }
}

pub(crate) struct StaticWebTagSymbolNamespace {
    pub web_tags: &'static [StaticWebTag],
}

impl StaticWebTagSymbolNamespace {}

impl dyn_symbol::namespace::Static for StaticWebTagSymbolNamespace {
    fn namespace_name(&self) -> &str {
        "web-tag"
    }

    fn symbol_name(&self, id: u32) -> &str {
        self.web_tags[id as usize].name
    }
}
