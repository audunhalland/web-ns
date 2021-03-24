#[macro_export]
macro_rules! attr_flags {
    ($val:expr, $flag:ident) => {
        crate::attr::attr_type::AttrType($val | crate::attr::attr_type::flags::$flag)
    };
    ($val:expr, $flag:ident | $($rest:ident)|+) => {
        crate::attr_flags!($val | crate::attr::attr_type::flags::$flag, $($rest)|+)
    };
}

#[macro_export]
macro_rules! define_attrs {
    ($( ($name:ident, $attr:literal, $prop:literal, $($flags:ident)|+ ) ),* ) => {
        $(
            mod $name {
                pub const ATTRIBUTE: &str = $attr;
                pub const PROPERTY: &str = $prop;

                pub const INTERNAL_ATTR: crate::attr::attr_impl::InternalAttr = crate::attr::attr_impl::InternalAttr {
                    attribute: ATTRIBUTE,
                    property: PROPERTY,
                    attr_type: crate::attr_flags!(0, $($flags)|+),
                };
            }
        )*

        pub fn internal_attr_by_name(attribute: &str) -> Option<&'static crate::attr::attr_impl::InternalAttr> {
            match attribute {
                $(
                    $name::ATTRIBUTE => Some(&$name::INTERNAL_ATTR),
                )*
                _ => None
            }
        }

        pub fn internal_attr_by_property(property: &str) -> Option<&'static crate::attr::attr_impl::InternalAttr> {
            match property {
                $(
                    $name::PROPERTY => Some(&$name::INTERNAL_ATTR),
                )*
                _ => None
            }
        }
    };
}
