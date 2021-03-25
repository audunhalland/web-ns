//!
//! Wrapper types for unicase so as to avoid any unsafe code.
//!

use phf_shared::{FmtConst, PhfHash};

pub struct AsciiUniCaseSlice<'a>(unicase::UniCase<&'a str>);

impl<'a> AsciiUniCaseSlice<'a> {
    pub const fn new(str: &'a str) -> Self {
        Self(unicase::UniCase::ascii(str))
    }
}

impl<'a> PartialEq<AsciiUniCaseSlice<'_>> for AsciiUniCaseSlice<'a> {
    #[inline]
    fn eq(&self, rhs: &AsciiUniCaseSlice<'_>) -> bool {
        self.0.eq(&rhs.0)
    }
}
impl<'a> Eq for AsciiUniCaseSlice<'a> {}

impl<'a> std::hash::Hash for AsciiUniCaseSlice<'a> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<'a> PhfHash for AsciiUniCaseSlice<'a> {
    #[inline]
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.phf_hash(state);
    }
}

pub struct StaticAsciiUniCaseSlice(AsciiUniCaseSlice<'static>);

impl StaticAsciiUniCaseSlice {
    pub const fn new(str: &'static str) -> Self {
        Self(AsciiUniCaseSlice::new(str))
    }
}

impl<'a> PartialEq<StaticAsciiUniCaseSlice> for StaticAsciiUniCaseSlice {
    fn eq(&self, rhs: &StaticAsciiUniCaseSlice) -> bool {
        self.0.eq(&rhs.0)
    }
}
impl<'a> Eq for StaticAsciiUniCaseSlice {}

impl<'a> std::hash::Hash for StaticAsciiUniCaseSlice {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<'a> PhfHash for StaticAsciiUniCaseSlice {
    #[inline]
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.phf_hash(state);
    }
}

impl<'a> FmtConst for StaticAsciiUniCaseSlice {
    #[inline]
    fn fmt_const(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StaticAsciiUniCaseSlice::new(\"{}\")", self.0 .0)
    }
}

///
/// Now to the point:
///
/// StaticAsciiUniCaseSlice borrows as a &AsciiUniCaseSlice<'a> with generic lifetime 'a,
/// allowing `phf::Map::get` to take a &AsciiUniCaseSlice<'a> instead of the static version.
/// Internally, it will borrow its static, stored key as this type, since
/// `phf::Map<K, _>::get<T>(key: &T) where K: Borrow<T>`.
///
/// Using two different types solves the problem with ::get(key: &K) because K requires
/// a 'static &str reference, but we don't have that when we want to look up in the map.
///
impl<'a> std::borrow::Borrow<AsciiUniCaseSlice<'a>> for StaticAsciiUniCaseSlice {
    fn borrow(&self) -> &AsciiUniCaseSlice<'a> {
        &self.0
    }
}
