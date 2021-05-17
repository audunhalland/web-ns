pub type Flags = u32;

///
/// The runtime type of an attribute.
///
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AttrType(pub Flags);

impl AttrType {
    pub fn is_bool(self) -> bool {
        self.any(flags::BOOL)
    }

    pub fn any(self, flags: Flags) -> bool {
        self.0 & flags > 0
    }
}

pub mod flags {
    use super::Flags;

    /// A boolean property: false if omitted, true if specified without value.
    pub const BOOL: Flags = 0x1;

    /// The explicit value "true":
    pub const TRUE: Flags = 0x2;

    /// The explicit value "false":
    pub const FALSE: Flags = 0x4;

    /// Any string:
    pub const STRING: Flags = 0x8;

    // The empty string, a subtype of string
    pub const EMPTY_STRING: Flags = 0x10;

    // Any number:
    pub const NUMBER: Flags = 0x20;

    pub const COMMA_SEP: Flags = 0x100;
    pub const SPACE_SEP: Flags = 0x200;
    pub const COMMA_OR_SPACE_SEP: Flags = 0x400;
}
