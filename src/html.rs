pub mod attrs {
    crate::define_attrs!(
        (alink, "alink", "aLink"),
        (abbr, "abbr", "abbr")
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn attribute_by_name() {
        let internal_attr = super::attrs::internal_attr_by_name("alink").unwrap();
        assert_eq!(internal_attr.attribute, "alink");
        assert_eq!(internal_attr.property, "aLink");
    }
}
