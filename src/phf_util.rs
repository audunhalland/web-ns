use unicase::UniCase;

///
/// Helper to encapsulate the unsafe hack required to
/// look up with a map key that has a reference inside it.
///
pub fn phf_unicase_lookup(
    map: &phf::Map<UniCase<&'static str>, usize>,
    key: &str,
) -> Option<usize> {
    // safety: Given that `phf` does not allocate this should be safe.
    // It would need to use a global Mutex and leak this reference into a static, since `map` is immutable.
    // phf would never do that.
    let key: &'static str = unsafe { std::mem::transmute(key) };
    map.get(&UniCase::new(key)).map(|index| *index)
}
