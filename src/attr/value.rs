use super::attr_type::flags::*;
use super::attr_type::*;
use super::AttributeValue;

///
/// Parse an Option of a string-based attribute value.
/// None represents an attribute without a value, e.g. `<foo bar />`
///
pub fn parse_attribute<S>(
    value: Option<S>,
    attr_type: AttrType,
) -> Result<AttributeValue, crate::Error>
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
                Err(crate::Error::Dang)
            }
        }
        Some(string) => match string.as_ref() {
            "" => {
                if attr_type.is_bool() {
                    Ok(AttributeValue::True)
                } else if attr_type.any(EMPTY_STRING | STRING) {
                    Ok(AttributeValue::String(String::new()))
                } else {
                    Err(crate::Error::Dang)
                }
            }
            "true" => {
                if attr_type.any(BOOL | TRUE | STRING | NUMBER) {
                    Ok(AttributeValue::True)
                } else {
                    Err(crate::Error::Dang)
                }
            }
            "false" => {
                if attr_type.any(BOOL | TRUE | STRING | NUMBER) {
                    Ok(AttributeValue::False)
                } else {
                    Err(crate::Error::Dang)
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
                    Err(crate::Error::Dang)
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none() -> Option<&'static str> {
        None
    }

    #[test]
    fn parse_bool() {
        let boolean = AttrType(BOOL);

        assert_eq!(parse_attribute(none(), boolean), Ok(AttributeValue::True));
        assert_eq!(parse_attribute(Some(""), boolean), Ok(AttributeValue::True));
        assert_eq!(
            parse_attribute(Some("true"), boolean),
            Ok(AttributeValue::True)
        );
        assert_eq!(
            parse_attribute(Some("false"), boolean),
            Ok(AttributeValue::False)
        );
        assert!(parse_attribute(Some("t"), boolean).is_err());
    }

    #[test]
    fn parse_true_or_false() {
        let tf = AttrType(TRUE | FALSE);

        assert_eq!(parse_attribute(Some("true"), tf), Ok(AttributeValue::True));
        assert_eq!(
            parse_attribute(Some("false"), tf),
            Ok(AttributeValue::False)
        );
        assert!(parse_attribute(none(), tf).is_err());
        assert!(parse_attribute(Some("t"), tf).is_err());
    }

    #[test]
    fn parse_empty_string() {
        let es = AttrType(EMPTY_STRING);
        assert_eq!(
            parse_attribute(Some(""), es),
            Ok(AttributeValue::String("".to_string()))
        );

        assert!(parse_attribute(none(), es).is_err());
        assert!(parse_attribute(Some("true"), es).is_err());
        assert!(parse_attribute(Some("false"), es).is_err());
        assert!(parse_attribute(Some("t"), es).is_err());
    }

    #[test]
    fn parse_number() {
        let num = AttrType(NUMBER);
        assert_eq!(
            parse_attribute(Some("a"), num),
            Ok(AttributeValue::String("a".to_string()))
        );
        assert_eq!(parse_attribute(Some("true"), num), Ok(AttributeValue::True));

        assert!(parse_attribute(Some(""), num).is_err());
    }

    #[test]
    fn parse_space_sep() {
        let sep = AttrType(STRING | SPACE_SEP);
        assert_eq!(
            parse_attribute(Some("a"), sep),
            Ok(AttributeValue::String("a".to_string()))
        );
        assert_eq!(parse_attribute(Some("true"), sep), Ok(AttributeValue::True));
        assert_eq!(
            parse_attribute(Some("a b "), sep),
            Ok(AttributeValue::Multi(vec![
                "a".to_string(),
                "b".to_string()
            ]))
        );

        assert!(parse_attribute(none(), sep).is_err());
    }

    #[test]
    fn parse_comma_sep() {
        let sep = AttrType(STRING | COMMA_SEP);
        assert_eq!(
            parse_attribute(Some("a"), sep),
            Ok(AttributeValue::String("a".to_string()))
        );
        assert_eq!(parse_attribute(Some("true"), sep), Ok(AttributeValue::True));
        assert_eq!(
            parse_attribute(Some("a b "), sep),
            Ok(AttributeValue::String("a b".to_string()))
        );
        assert_eq!(
            parse_attribute(Some("a, b "), sep),
            Ok(AttributeValue::Multi(vec![
                "a".to_string(),
                "b".to_string()
            ]))
        );

        assert!(parse_attribute(none(), sep).is_err());
    }

    #[test]
    fn parse_comma_or_space_sep() {
        let sep = AttrType(STRING | COMMA_OR_SPACE_SEP);
        assert_eq!(
            parse_attribute(Some("a"), sep),
            Ok(AttributeValue::String("a".to_string()))
        );
        assert_eq!(parse_attribute(Some("true"), sep), Ok(AttributeValue::True));
        assert_eq!(
            parse_attribute(Some("a b "), sep),
            Ok(AttributeValue::Multi(vec![
                "a".to_string(),
                "b".to_string()
            ]))
        );
        assert_eq!(
            parse_attribute(Some("a, b "), sep),
            Ok(AttributeValue::Multi(vec![
                "a".to_string(),
                "b".to_string()
            ]))
        );

        assert!(parse_attribute(none(), sep).is_err());
    }
}
