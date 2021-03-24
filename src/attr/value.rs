use super::attr_type::*;
use super::AttributeValue;

pub fn encode<S>(value: Option<S>, attr_type: &AttrType) -> Result<AttributeValue, crate::Error>
where
    S: Into<String> + AsRef<str>,
{
    Err(crate::Error::Dang)
}
