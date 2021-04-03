use super::{AttributeValue, SerializedAttributeValue};

use super::attr_type::flags::*;
use super::attr_type::*;

///
/// Parse an Option of a string-based attribute value.
/// None represents an attribute without a value, e.g. `<foo bar />`
///
pub fn parse_attribute<S>(
    value: Option<S>,
    attr_type: AttrType,
) -> Result<AttributeValue, doml::Error>
where
    S: Into<String> + AsRef<str>,
{
    fn maybe_multi<'s>(it: impl Iterator<Item = &'s str>) -> AttributeValue {
        let strings: Vec<String> = it.filter(|s| !s.is_empty()).map(|s| s.into()).collect();
        if strings.len() == 0 {
            AttributeValue::String(String::new())
        } else if strings.len() == 1 {
            AttributeValue::String(strings.into_iter().next().unwrap())
        } else {
            AttributeValue::Multi(strings)
        }
    }

    match value {
        None => {
            if attr_type.is_bool() {
                Ok(AttributeValue::True)
            } else {
                Err(doml::Error::InvalidAttributeValue)
            }
        }
        Some(string) => match string.as_ref() {
            "" => {
                if attr_type.is_bool() {
                    Ok(AttributeValue::True)
                } else if attr_type.any(EMPTY_STRING | STRING) {
                    Ok(AttributeValue::String(String::new()))
                } else {
                    Err(doml::Error::InvalidAttributeValue)
                }
            }
            "true" => {
                if attr_type.any(BOOL | TRUE | STRING | NUMBER) {
                    Ok(AttributeValue::True)
                } else {
                    Err(doml::Error::InvalidAttributeValue)
                }
            }
            "false" => {
                if attr_type.any(BOOL | TRUE | STRING | NUMBER) {
                    Ok(AttributeValue::False)
                } else {
                    Err(doml::Error::InvalidAttributeValue)
                }
            }
            _ => {
                let str = string.as_ref();

                if attr_type.any(SPACE_SEP) {
                    Ok(maybe_multi(str.split(' ')))
                } else if attr_type.any(COMMA_SEP) {
                    Ok(maybe_multi(str.split(',').map(str::trim)))
                } else if attr_type.any(COMMA_OR_SPACE_SEP) {
                    Ok(maybe_multi(
                        str.split(|c| c == ' ' || c == ',').map(str::trim),
                    ))
                } else if attr_type.any(STRING | NUMBER) {
                    Ok(AttributeValue::String(string.into()))
                } else {
                    Err(doml::Error::InvalidAttributeValue)
                }
            }
        },
    }
}

pub fn serialize_attribute_value(
    value: &AttributeValue,
    attr_type: AttrType,
) -> SerializedAttributeValue {
    match value {
        AttributeValue::False => {
            if attr_type.is_bool() {
                SerializedAttributeValue::Omitted
            } else {
                SerializedAttributeValue::String("false".to_string())
            }
        }
        AttributeValue::True => {
            if attr_type.is_bool() {
                SerializedAttributeValue::Empty
            } else {
                SerializedAttributeValue::String("true".to_string())
            }
        }
        AttributeValue::String(string) => {
            if attr_type.is_bool() {
                SerializedAttributeValue::Empty
            } else {
                SerializedAttributeValue::String(string.clone())
            }
        }
        AttributeValue::Multi(vec) => {
            if attr_type.is_bool() {
                SerializedAttributeValue::Empty
            } else if attr_type.any(COMMA_SEP) {
                SerializedAttributeValue::String(vec.join(", "))
            } else if attr_type.any(COMMA_OR_SPACE_SEP) {
                // FIXME: Check COMMA_OR_SPACE_SEP semantics..
                SerializedAttributeValue::String(vec.join(" "))
            } else {
                // Correctness: Value should have been deserialized using the
                // correct attr_type
                SerializedAttributeValue::String(vec.join(" "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none() -> Option<&'static str> {
        None
    }

    fn assert_parse_serialize_ok(
        attr_type: AttrType,
        input: Option<&str>,
        expected_value: AttributeValue,
        expected_serialization: SerializedAttributeValue,
    ) {
        let attribute_value = parse_attribute(input, attr_type).unwrap();
        assert_eq!(attribute_value, expected_value);

        let serialized = serialize_attribute_value(&attribute_value, attr_type);
        assert_eq!(serialized, expected_serialization);
    }

    #[test]
    fn parse_bool() {
        let boolean = AttrType(BOOL);

        assert_parse_serialize_ok(
            boolean,
            None,
            AttributeValue::True,
            SerializedAttributeValue::Empty,
        );
        assert_parse_serialize_ok(
            boolean,
            Some(""),
            AttributeValue::True,
            SerializedAttributeValue::Empty,
        );
        assert_parse_serialize_ok(
            boolean,
            none(),
            AttributeValue::True,
            SerializedAttributeValue::Empty,
        );
        assert_parse_serialize_ok(
            boolean,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::Empty,
        );
        assert_parse_serialize_ok(
            boolean,
            Some("false"),
            AttributeValue::False,
            SerializedAttributeValue::Omitted,
        );

        assert!(parse_attribute(Some("t"), boolean).is_err());
    }

    #[test]
    fn parse_true_or_false() {
        let tf = AttrType(TRUE | FALSE);

        assert_parse_serialize_ok(
            tf,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::String("true".to_string()),
        );
        assert_parse_serialize_ok(
            tf,
            Some("false"),
            AttributeValue::False,
            SerializedAttributeValue::String("false".to_string()),
        );

        assert!(parse_attribute(none(), tf).is_err());
        assert!(parse_attribute(Some("t"), tf).is_err());
    }

    #[test]
    fn parse_empty_string() {
        let es = AttrType(EMPTY_STRING);

        assert_parse_serialize_ok(
            es,
            Some(""),
            AttributeValue::String(String::new()),
            SerializedAttributeValue::String(String::new()),
        );

        assert!(parse_attribute(none(), es).is_err());
        assert!(parse_attribute(Some("true"), es).is_err());
        assert!(parse_attribute(Some("false"), es).is_err());
        assert!(parse_attribute(Some("t"), es).is_err());
    }

    #[test]
    fn parse_number() {
        let num = AttrType(NUMBER);

        assert_parse_serialize_ok(
            num,
            Some("a"),
            AttributeValue::String("a".to_string()),
            SerializedAttributeValue::String("a".to_string()),
        );
        assert_parse_serialize_ok(
            num,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::String("true".to_string()),
        );

        assert!(parse_attribute(Some(""), num).is_err());
    }

    #[test]
    fn parse_space_sep() {
        let sep = AttrType(STRING | SPACE_SEP);

        assert_parse_serialize_ok(
            sep,
            Some("a"),
            AttributeValue::String("a".to_string()),
            SerializedAttributeValue::String("a".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::String("true".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("a b "),
            AttributeValue::Multi(vec!["a".to_string(), "b".to_string()]),
            SerializedAttributeValue::String("a b".to_string()),
        );

        assert!(parse_attribute(none(), sep).is_err());
    }

    #[test]
    fn parse_comma_sep() {
        let sep = AttrType(STRING | COMMA_SEP);

        assert_parse_serialize_ok(
            sep,
            Some("a"),
            AttributeValue::String("a".to_string()),
            SerializedAttributeValue::String("a".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::String("true".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("a b"),
            AttributeValue::String("a b".to_string()),
            SerializedAttributeValue::String("a b".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("a, b "),
            AttributeValue::Multi(vec!["a".to_string(), "b".to_string()]),
            SerializedAttributeValue::String("a, b".to_string()),
        );

        assert!(parse_attribute(none(), sep).is_err());
    }

    #[test]
    fn parse_comma_or_space_sep() {
        let sep = AttrType(STRING | COMMA_OR_SPACE_SEP);

        assert_parse_serialize_ok(
            sep,
            Some("a"),
            AttributeValue::String("a".to_string()),
            SerializedAttributeValue::String("a".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("true"),
            AttributeValue::True,
            SerializedAttributeValue::String("true".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("a b"),
            AttributeValue::Multi(vec!["a".to_string(), "b".to_string()]),
            SerializedAttributeValue::String("a b".to_string()),
        );
        assert_parse_serialize_ok(
            sep,
            Some("a, b "),
            AttributeValue::Multi(vec!["a".to_string(), "b".to_string()]),
            SerializedAttributeValue::String("a b".to_string()),
        );

        assert!(parse_attribute(none(), sep).is_err());
    }
}
