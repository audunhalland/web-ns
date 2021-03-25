//!
//! Wrapper types for unicase so as to avoid any unsafe code.
//!

use phf_shared::{FmtConst, PhfHash};
use unicase::UniCase;

///
/// UniCase wrapper suitable as a key for phf::Map.
/// For the reason why, see the Borrow impl for it.
///
pub struct StaticUniCase(UniCase<&'static str>);

impl StaticUniCase {
    pub const fn new(str: &'static str) -> Self {
        Self(UniCase::ascii(str))
    }
}

impl<'a> PartialEq<StaticUniCase> for StaticUniCase {
    fn eq(&self, rhs: &StaticUniCase) -> bool {
        self.0.eq(&rhs.0)
    }
}
impl<'a> Eq for StaticUniCase {}

impl<'a> std::hash::Hash for StaticUniCase {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<'a> PhfHash for StaticUniCase {
    #[inline]
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.phf_hash(state);
    }
}

impl<'a> FmtConst for StaticUniCase {
    #[inline]
    fn fmt_const(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StaticUniCase::new(\"{}\")", &self.0)
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
impl<'a> std::borrow::Borrow<UniCase<&'a str>> for StaticUniCase {
    fn borrow(&self) -> &UniCase<&'a str> {
        &self.0
    }
}
