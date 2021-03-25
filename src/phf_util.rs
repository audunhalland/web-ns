use unicase::UniCase;

pub fn phf_str_lookup(map: &phf::Map<&'static str, usize>, key: &str) -> Option<usize> {
    // safety: Given that `phf` does not allocate this should be safe.
    // It would need to use a global Mutex and leak this reference into a static, since `map` is immutable.
    // phf would never do that.
    let key: &'static str = unsafe { std::mem::transmute(key) };
    map.get(&key).map(|index| *index)
}

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
