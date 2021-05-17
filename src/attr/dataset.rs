use super::attr_type::flags;
use super::attr_type::AttrType;

#[derive(Clone)]
pub struct DataAttr {
    strbuf: String,
    buf_property_start: usize,
    attr_type: AttrType,
}

impl DataAttr {
    pub(crate) fn parse_attribute(name: &str) -> Result<Self, crate::Error> {
        if name.len() > 5 && unicase::UniCase::new(&name[..5]) == unicase::UniCase::new("data-") {
            let strbuf = format!(
                "data-{}data{}{}",
                &name[5..],
                name.chars().nth(5).unwrap().to_uppercase(),
                &name[6..]
            );
            Ok(Self {
                strbuf,
                buf_property_start: name.len(),
                attr_type: AttrType(flags::STRING),
            })
        } else {
            Err(crate::Error::InvalidAttribute)
        }
    }

    pub(crate) fn parse_property(name: &str) -> Result<Self, crate::Error> {
        if name.len() > 4 && name.starts_with("data") {
            let strbuf = format!(
                "data-{}{}{}",
                name.chars().nth(4).unwrap().to_lowercase(),
                &name[5..],
                name
            );
            Ok(Self {
                strbuf,
                buf_property_start: name.len() + 1,
                attr_type: AttrType(flags::STRING),
            })
        } else {
            Err(crate::Error::InvalidAttribute)
        }
    }
}

impl crate::LocalName for DataAttr {
    fn local_name(&self) -> &str {
        &self.strbuf[..self.buf_property_start]
    }
}

impl crate::attr::Attribute for DataAttr {
    fn attr_type(&self) -> crate::attr::attr_type::AttrType {
        self.attr_type
    }
}

impl crate::PropertyName for DataAttr {
    fn property_name(&self) -> &str {
        &self.strbuf[self.buf_property_start..]
    }
}

impl PartialEq for DataAttr {
    fn eq(&self, rhs: &Self) -> bool {
        use crate::LocalName;
        self.local_name() == rhs.local_name()
    }
}

impl Eq for DataAttr {}

impl Ord for DataAttr {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        use crate::LocalName;
        self.local_name().cmp(rhs.local_name())
    }
}

impl PartialOrd for DataAttr {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl std::hash::Hash for DataAttr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use crate::LocalName;
        self.local_name().hash(state);
    }
}

impl std::fmt::Debug for DataAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::LocalName;
        write!(f, "{}", self.local_name())
    }
}
