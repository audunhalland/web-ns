use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UniCaseKey<'a>(pub unicase::Ascii<&'a str>);

impl<'a> UniCaseKey<'a> {
    pub const fn new(str: &'a str) -> Self {
        Self(unicase::Ascii::new(str))
    }
}

impl<'a> phf_shared::FmtConst for UniCaseKey<'a> {
    fn fmt_const(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UniCaseKey::new({:?})", self.0.as_ref())
    }
}

impl<'a> phf_shared::PhfHash for UniCaseKey<'a> {
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
